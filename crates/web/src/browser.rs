use rustenium_core::capabilities::Capabilities;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct ChromeCapabilities {
    #[serde(rename = "goog:chromeOptions")]
    pub chrome_options: ChromeOptions,
}

impl Default for ChromeCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromeCapabilities {
    pub fn new() -> Self {
        Self {
            chrome_options: ChromeOptions::new(),
        }
    }
}

#[derive(Serialize)]
pub struct ChromeOptions {
    pub args: Vec<String>,
}

impl Default for ChromeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromeOptions {
    pub fn new() -> Self {
        Self {
            args: vec!["--headless".to_string()],
        }
    }
}

pub enum BrowserCapabilities {
    Chrome(ChromeCapabilities),
}

impl Capabilities for BrowserCapabilities {
    fn to_value(&self) -> Value {
        match self {
            BrowserCapabilities::Chrome(caps) => json!({
                "browserName": "chrome",
                "goog:chromeOptions": caps.chrome_options
            }),
        }
    }
}
