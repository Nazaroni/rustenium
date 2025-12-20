pub mod browser;
pub mod element;

use rustenium_core::capabilities::Capabilities;
use rustenium_core::client::{WebDriverHttpClient, WebDriverResult};
use rustenium_core::session::WebDriverSession;
use browser::BrowserCapabilities;
use element::WebElement;

pub struct WebDriver {
    session: WebDriverSession,
}

impl WebDriver {
    pub async fn new(base_url: &str, capabilities: BrowserCapabilities) -> WebDriverResult<Self> {
        let client = WebDriverHttpClient::new(base_url);
        let session = WebDriverSession::create(client, capabilities.to_value()).await?;
        Ok(Self { session })
    }

    pub async fn goto(&self, url: &str) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/url", self.session.session_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({ "url": url })).await?;
        Ok(())
    }

    pub async fn title(&self) -> WebDriverResult<String> {
        let endpoint = format!("/session/{}/title", self.session.session_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_str().unwrap_or("").to_string())
    }

    pub async fn find_element(&self, selector: &str) -> WebDriverResult<WebElement> {
        let endpoint = format!("/session/{}/element", self.session.session_id);
        let params = serde_json::json!({ "using": "css selector", "value": selector });
        let response: serde_json::Value = self.session.client.post(&endpoint, &params).await?;
        let element_id = response["value"]["element-6066-11e4-a52e-4f735466cecf"].as_str().unwrap_or("").to_string();
        Ok(WebElement {
            element_id,
            session: self.session.clone(),
        })
    }

    pub async fn quit(self) -> WebDriverResult<()> {
        self.session.delete().await
    }
}