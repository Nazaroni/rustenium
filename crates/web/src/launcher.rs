use std::process::{Child, Command};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LauncherError {
    #[error("Failed to spawn driver process: {0}")]
    Spawn(String),
    #[error("Driver failed to start within timeout")]
    Timeout,
    #[error("Port {0} is already in use")]
    PortInUse(u16),
}

/// Simple port check without external dependencies
fn is_port_in_use_simple(port: u16) -> bool {
    // On Unix-like systems, check if the port is in use
    #[cfg(target_os = "linux")]
    {
        Command::new("sh")
            .arg("-c")
            .arg(format!("lsof -i :{} 2>/dev/null | grep LISTEN >/dev/null 2>&1", port))
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("sh")
            .arg("-c")
            .arg(format!("lsof -i :{} 2>/dev/null | grep LISTEN >/dev/null 2>&1", port))
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        // For other platforms, assume port is not in use
        false
    }
}

/// Launches and manages a WebDriver server process
pub struct DriverLauncher {
    driver_path: String,
    port: u16,
    timeout_ms: u64,
    headless: bool,
}

impl DriverLauncher {
    /// Create a new DriverLauncher with default settings
    pub fn new() -> Self {
        Self {
            driver_path: "chromedriver".to_string(),
            port: 9515,
            timeout_ms: 5000,
            headless: true,
        }
    }

    /// Set the path to the driver binary
    pub fn with_driver_path(&mut self, path: &str) {
        self.driver_path = path.to_string();
    }

    /// Set the port to use for the driver
    pub fn with_port(&mut self, port: u16) {
        self.port = port;
    }

    /// Set the timeout for driver startup (in milliseconds)
    pub fn with_timeout(&mut self, timeout_ms: u64) {
        self.timeout_ms = timeout_ms;
    }

    /// Set headless mode
    pub fn with_headless(&mut self, headless: bool) {
        self.headless = headless;
    }

    /// Check if the port is already in use by another process
    fn is_port_in_use(port: u16) -> bool {
        // On Unix-like systems, check if the port is in use
        is_port_in_use_simple(port)
    }

    /// Start the driver process
    pub fn start(&self) -> Result<Child, LauncherError> {
        // Check if port is already in use
        if Self::is_port_in_use(self.port) {
            return Err(LauncherError::PortInUse(self.port));
        }

        let mut cmd = Command::new(&self.driver_path);
        cmd.arg(format!("--port={}", self.port));

        if self.headless {
            // Chrome-specific flag for headless mode
            cmd.arg("--headless=new");
        }

        let mut child = cmd.spawn().map_err(|e| {
            LauncherError::Spawn(format!("Failed to spawn {}: {}", self.driver_path, e))
        })?;

        // Wait for driver to be ready
        if !self.wait_for_ready(5000) {
            // Try to kill the process if it failed to start
            let _ = child.kill();
            return Err(LauncherError::Timeout);
        }

        Ok(child)
    }

    /// Wait for the driver to be ready (port is listening)
    fn wait_for_ready(&self, timeout_ms: u64) -> bool {
        let start = std::time::Instant::now();
        while start.elapsed().as_millis() < (timeout_ms as u128) {
            if Self::is_port_in_use(self.port) {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        false
    }
}

impl Default for DriverLauncher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launcher_creation() {
        let launcher = DriverLauncher::new();
        assert_eq!(launcher.driver_path, "chromedriver");
        assert_eq!(launcher.port, 9515);
    }

    #[test]
    fn test_launcher_builder_pattern() {
        let mut launcher = DriverLauncher::new();
        launcher.with_driver_path("/path/to/chromedriver");
        launcher.with_port(4444);
        launcher.with_headless(false);

        assert_eq!(launcher.driver_path, "/path/to/chromedriver");
        assert_eq!(launcher.port, 4444);
        assert!(!launcher.headless);
    }
}
