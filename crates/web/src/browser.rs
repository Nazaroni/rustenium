use rustenium_core::capabilities::Capabilities;
use serde::Serialize;
use serde_json::{json, Value};

/// Browser enumeration for different browser support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Browser {
    Chrome,
    Firefox,
    Safari,
    Edge,
}

/// Chrome-specific options
#[derive(Serialize, Clone)]
pub struct ChromeOptions {
    #[serde(rename = "args")]
    pub arguments: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
}

impl Default for ChromeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromeOptions {
    /// Create new Chrome options with default settings (headless mode)
    pub fn new() -> Self {
        Self {
            arguments: vec!["--headless=new".to_string(), "--no-sandbox".to_string()],
            binary: None,
            extensions: None,
        }
    }

    /// Add an argument to the Chrome command line
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.arguments.push(arg.into());
        self
    }

    /// Set the path to the Chrome binary
    pub fn binary(mut self, path: impl Into<String>) -> Self {
        self.binary = Some(path.into());
        self
    }

    /// Set headless mode argument
    pub fn headless(mut self, enabled: bool) -> Self {
        if enabled {
            // Remove existing headless arg and add new one
            self.arguments = vec!["--headless=new".to_string(), "--no-sandbox".to_string()];
        }
        self
    }
}

/// Chrome capabilities
#[derive(Serialize, Clone)]
pub struct ChromeCapabilities {
    #[serde(rename = "goog:chromeOptions")]
    pub chrome_options: ChromeOptions,
}

impl Default for ChromeCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl Capabilities for ChromeCapabilities {
    fn to_value(&self) -> Value {
        json!({
            "browserName": "chrome",
            "goog:chromeOptions": self.chrome_options
        })
    }
}

impl ChromeCapabilities {
    /// Create new Chrome capabilities with default options
    pub fn new() -> Self {
        Self {
            chrome_options: ChromeOptions::new(),
        }
    }

    /// Set Chrome options
    pub fn with_options(mut self, options: ChromeOptions) -> Self {
        self.chrome_options = options;
        self
    }

    /// Set headless mode
    pub fn headless(mut self) -> Self {
        self.chrome_options = ChromeOptions::new();
        self
    }

    /// Convert capabilities to JSON value (for compatibility)
    pub fn to_value(&self) -> Value {
        json!({
            "browserName": "chrome",
            "goog:chromeOptions": self.chrome_options
        })
    }
}

/// Firefox capabilities
#[derive(Serialize, Clone)]
pub struct FirefoxCapabilities {
    #[serde(rename = "moz:firefoxOptions")]
    pub firefox_options: FirefoxOptions,
}

impl Default for FirefoxCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl Capabilities for FirefoxCapabilities {
    fn to_value(&self) -> Value {
        json!({
            "browserName": "firefox",
            "moz:firefoxOptions": self.firefox_options
        })
    }
}

impl FirefoxCapabilities {
    /// Create new Firefox capabilities with default options
    pub fn new() -> Self {
        Self {
            firefox_options: FirefoxOptions::new(),
        }
    }

    /// Set Firefox options
    pub fn with_options(mut self, options: FirefoxOptions) -> Self {
        self.firefox_options = options;
        self
    }

    /// Set headless mode
    pub fn headless(mut self) -> Self {
        self.firefox_options = FirefoxOptions::new();
        self
    }

    /// Convert capabilities to JSON value (for compatibility)
    pub fn to_value(&self) -> Value {
        json!({
            "browserName": "firefox",
            "moz:firefoxOptions": self.firefox_options
        })
    }
}

/// Firefox-specific options
#[derive(Serialize, Clone)]
pub struct FirefoxOptions {
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
}

impl Default for FirefoxOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl FirefoxOptions {
    /// Create new Firefox options with default settings (headless mode)
    pub fn new() -> Self {
        Self {
            args: vec!["-headless".to_string()],
            binary: None,
        }
    }

    /// Add an argument to the Firefox command line
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Set the path to the Firefox binary
    pub fn binary(mut self, path: impl Into<String>) -> Self {
        self.binary = Some(path.into());
        self
    }
}

/// Safari capabilities
#[derive(Serialize, Clone)]
pub struct SafariCapabilities {
    #[serde(rename = "safari:options")]
    pub safari_options: SafariOptions,
}

impl Default for SafariCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl Capabilities for SafariCapabilities {
    fn to_value(&self) -> Value {
        json!({
            "browserName": "safari",
            "safari:options": self.safari_options
        })
    }
}

impl SafariCapabilities {
    /// Create new Safari capabilities with default options
    pub fn new() -> Self {
        Self {
            safari_options: SafariOptions::new(),
        }
    }
}

/// Safari-specific options
#[derive(Serialize, Clone)]
pub struct SafariOptions {
    pub automatic_profiling: Option<bool>,
}

impl Default for SafariOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl SafariOptions {
    /// Create new Safari options with default settings
    pub fn new() -> Self {
        Self { automatic_profiling: None }
    }
}

/// Edge capabilities
#[derive(Serialize, Clone)]
pub struct EdgeCapabilities {
    #[serde(rename = "ms:edgeOptions")]
    pub edge_options: EdgeOptions,
}

impl Default for EdgeCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl Capabilities for EdgeCapabilities {
    fn to_value(&self) -> Value {
        json!({
            "browserName": "edge",
            "ms:edgeOptions": self.edge_options
        })
    }
}

impl EdgeCapabilities {
    /// Create new Edge capabilities with default options
    pub fn new() -> Self {
        Self {
            edge_options: EdgeOptions::new(),
        }
    }
}

/// Edge-specific options
#[derive(Serialize, Clone)]
pub struct EdgeOptions {
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
}

impl Default for EdgeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl EdgeOptions {
    /// Create new Edge options with default settings (headless mode)
    pub fn new() -> Self {
        Self {
            args: vec!["--headless".to_string()],
            binary: None,
        }
    }

    /// Add an argument to the Edge command line
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Set the path to the Edge binary
    pub fn binary(mut self, path: impl Into<String>) -> Self {
        self.binary = Some(path.into());
        self
    }
}

/// Browser capabilities enum
pub enum BrowserCapabilities {
    Chrome(ChromeCapabilities),
    Firefox(FirefoxCapabilities),
    Safari(SafariCapabilities),
    Edge(EdgeCapabilities),
}

impl Capabilities for BrowserCapabilities {
    fn to_value(&self) -> Value {
        match self {
            BrowserCapabilities::Chrome(caps) => json!({
                "browserName": "chrome",
                "goog:chromeOptions": caps.chrome_options
            }),
            BrowserCapabilities::Firefox(caps) => json!({
                "browserName": "firefox",
                "moz:firefoxOptions": caps.firefox_options
            }),
            BrowserCapabilities::Safari(caps) => json!({
                "browserName": "safari",
                "safari: safariOptions": caps.safari_options
            }),
            BrowserCapabilities::Edge(caps) => json!({
                "browserName": "edge",
                "ms:edgeOptions": caps.edge_options
            }),
        }
    }
}

impl BrowserCapabilities {
    /// Convert capabilities to JSON value (for compatibility)
    pub fn to_value(&self) -> Value {
        Capabilities::to_value(self)
    }

    /// Get the browser type
    pub fn browser(&self) -> Browser {
        match self {
            BrowserCapabilities::Chrome(_) => Browser::Chrome,
            BrowserCapabilities::Firefox(_) => Browser::Firefox,
            BrowserCapabilities::Safari(_) => Browser::Safari,
            BrowserCapabilities::Edge(_) => Browser::Edge,
        }
    }

    /// Set browser to headless mode
    pub fn headless(mut self) -> Self {
        match &mut self {
            BrowserCapabilities::Chrome(caps) => {
                caps.chrome_options = ChromeOptions::new();
            }
            BrowserCapabilities::Firefox(caps) => {
                caps.firefox_options = FirefoxOptions::new();
            }
            BrowserCapabilities::Edge(caps) => {
                caps.edge_options = EdgeOptions::new();
            }
            BrowserCapabilities::Safari(_) => {
                // Safari headless requires different handling
            }
        }
        self
    }
}
