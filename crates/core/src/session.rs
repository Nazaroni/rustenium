use crate::client::{WebDriverHttpClient, WebDriverResult};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct WebDriverSession {
    pub client: WebDriverHttpClient,
    pub session_id: String,
}

impl WebDriverSession {
    pub async fn create(client: WebDriverHttpClient, capabilities: Value) -> WebDriverResult<Self> {
        let response: Value = client.post("/session", &serde_json::json!({ "capabilities": capabilities })).await?;
        let session_id = response["value"]["sessionId"].as_str().unwrap_or("").to_string();
        Ok(Self {
            client,
            session_id,
        })
    }

    pub async fn delete(self) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}", self.session_id);
        self.client.delete(&endpoint).await
    }
}
