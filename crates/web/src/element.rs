use rustenium_core::client::WebDriverResult;
use rustenium_core::session::WebDriverSession;

#[derive(Clone)]
pub struct WebElement {
    pub element_id: String,
    pub session: WebDriverSession,
}

impl WebElement {
    pub async fn send_keys(&self, keys: &str) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/element/{}/value", self.session.session_id, self.element_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({ "text": keys })).await?;
        Ok(())
    }

    pub async fn click(&self) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/element/{}/click", self.session.session_id, self.element_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({})).await?;
        Ok(())
    }
}
