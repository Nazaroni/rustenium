pub mod browser;
pub mod element;
pub mod launcher;
pub mod prelude;

use rustenium_core::client::{WebDriverHttpClient, WebDriverResult};
use rustenium_core::session::WebDriverSession;
pub use browser::{Browser, BrowserCapabilities, ChromeCapabilities, ChromeOptions};
pub use browser::{FirefoxCapabilities, FirefoxOptions};
pub use browser::{SafariCapabilities, SafariOptions};
pub use browser::{EdgeCapabilities, EdgeOptions};
pub use launcher::DriverLauncher;
use element::WebElement;
use std::time::Duration;

/// Configuration for WebDriver instance
#[derive(Debug, Clone)]
pub struct WebDriverConfig {
    pub base_url: String,
    pub port: u16,
    pub headless: bool,
}

impl Default for WebDriverConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost".to_string(),
            port: 9515,
            headless: true,
        }
    }
}

impl WebDriverConfig {
    /// Create a new config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL (including protocol)
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the port for the WebDriver server
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Enable or disable headless mode
    pub fn with_headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }
}

pub struct WebDriver {
    session: WebDriverSession,
    _driver_process: Option<DriverLauncher>,
}

impl WebDriver {
    /// Create a new WebDriver instance with the given config
    pub async fn new(config: &WebDriverConfig) -> WebDriverResult<Self> {
        let client = WebDriverHttpClient::new(&config.base_url);
        // Create capabilities with headless mode
        let mut chrome_caps = ChromeCapabilities::new();
        chrome_caps.chrome_options.arguments = vec![
            "--headless=new".to_string(),
            "--no-sandbox".to_string()
        ];
        let session = WebDriverSession::create(client, chrome_caps.to_value()).await?;
        Ok(Self {
            session,
            _driver_process: None,
        })
    }

    /// Create a new Chrome WebDriver instance with default settings
    pub async fn new_chrome() -> WebDriverResult<Self> {
        Self::new(&WebDriverConfig::default()).await
    }

    /// Create a new Chrome WebDriver instance with custom options
    pub async fn new_chrome_with_options(options: ChromeOptions) -> WebDriverResult<Self> {
        let config = WebDriverConfig::default();
        let client = WebDriverHttpClient::new(&config.base_url);
        // Create capabilities with custom options and then set headless
        let mut chrome_caps = ChromeCapabilities::new().with_options(options);
        // Set headless args manually
        chrome_caps.chrome_options.arguments = vec![
            "--headless=new".to_string(),
            "--no-sandbox".to_string()
        ];
        let session = WebDriverSession::create(client, chrome_caps.to_value()).await?;
        Ok(Self {
            session,
            _driver_process: None,
        })
    }

    /// Create a new Firefox WebDriver instance
    pub async fn new_firefox() -> WebDriverResult<Self> {
        let config = WebDriverConfig::default();
        let client = WebDriverHttpClient::new(&config.base_url);
        let capabilities = FirefoxCapabilities::new().headless();
        let session = WebDriverSession::create(client, capabilities.to_value()).await?;
        Ok(Self {
            session,
            _driver_process: None,
        })
    }

    /// Navigate to a URL
    pub async fn goto(&self, url: &str) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/url", self.session.session_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({ "url": url })).await?;
        Ok(())
    }

    /// Get the current page title
    pub async fn title(&self) -> WebDriverResult<String> {
        let endpoint = format!("/session/{}/title", self.session.session_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_str().unwrap_or("").to_string())
    }

    /// Go back to the previous page
    pub async fn back(&self) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/back", self.session.session_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Go forward to the next page
    pub async fn forward(&self) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/forward", self.session.session_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Refresh the current page
    pub async fn refresh(&self) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/refresh", self.session.session_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Find an element by CSS selector
    pub async fn find_element(&self, selector: &str) -> WebDriverResult<WebElement> {
        let endpoint = format!("/session/{}/element", self.session.session_id);
        let params = serde_json::json!({ "using": "css selector", "value": selector });
        let response: serde_json::Value = self.session.client.post(&endpoint, &params).await?;
        let element_id = response["value"]["element-6066-11e4-a52e-4f735466cecf"]
            .as_str()
            .unwrap_or("")
            .to_string();
        Ok(WebElement {
            element_id,
            session: self.session.clone(),
        })
    }

    /// Get all elements matching the selector
    pub async fn find_elements(&self, selector: &str) -> WebDriverResult<Vec<WebElement>> {
        let endpoint = format!("/session/{}/elements", self.session.session_id);
        let params = serde_json::json!({ "using": "css selector", "value": selector });
        let response: serde_json::Value = self.session.client.post(&endpoint, &params).await?;
        let elements: Vec<WebElement> = response["value"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|e| {
                        let id = e["element-6066-11e4-a52e-4f735466cecf"]
                            .as_str()
                            .unwrap_or("")
                            .to_string();
                        WebElement {
                            element_id: id,
                            session: self.session.clone(),
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();
        Ok(elements)
    }

    /// Quit the browser and end the session
    pub async fn quit(self) -> WebDriverResult<()> {
        self.session.delete().await
    }

    /// Take a screenshot of the current page
    pub async fn screenshot(&self) -> WebDriverResult<String> {
        let endpoint = format!("/session/{}/screenshot", self.session.session_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_str().unwrap_or("").to_string())
    }

    /// Get the session ID
    pub fn session_id(&self) -> &str {
        &self.session.session_id
    }

    /// Wait for an element to be present in the DOM
    pub async fn wait_for_element(
        &self,
        selector: &str,
        timeout: Duration,
    ) -> WebDriverResult<Option<WebElement>> {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() >= timeout {
                break;
            }
            match self.find_element(selector).await {
                Ok(element) => return Ok(Some(element)),
                Err(_) => {
                    // Continue waiting
                }
            }
        }

        Ok(None)
    }

    /// Wait for an element to be visible
    pub async fn wait_until_visible(
        &self,
        selector: &str,
        timeout: Duration,
    ) -> WebDriverResult<Option<WebElement>> {
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() >= timeout {
                break;
            }
            if let Ok(element) = self.find_element(selector).await {
                // Check visibility by getting element rect and checking if it has positive dimensions
                let rect = element.rect().await.ok();
                if rect.as_ref().map(|r| r.is_visible()).unwrap_or(false) {
                    return Ok(Some(element));
                }
            }
        }

        Ok(None)
    }

    /// Wait for a condition to be true
    pub async fn wait_until<F, Fut>(&self, mut condition: F, timeout: Duration) -> WebDriverResult<bool>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() >= timeout {
                break;
            }
            if condition().await {
                return Ok(true);
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok(false)
    }
}

/// Wait utilities for WebDriver
pub mod waits {
    use super::*;

    /// Generic wait function that polls a closure until it returns true or times out
    pub async fn wait_until<F, Fut>(driver: &WebDriver, condition: F, timeout: Duration) -> WebDriverResult<bool>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = bool>,
    {
        driver.wait_until(condition, timeout).await
    }

    /// Wait for an element to be present
    pub async fn until_element_present(
        driver: &WebDriver,
        selector: &str,
        timeout: Duration,
    ) -> WebDriverResult<Option<WebElement>> {
        driver.wait_for_element(selector, timeout).await
    }
}