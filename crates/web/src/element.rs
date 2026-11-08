use rustenium_core::client::WebDriverResult;
use rustenium_core::session::WebDriverSession;

#[derive(Clone)]
pub struct WebElement {
    pub element_id: String,
    pub session: WebDriverSession,
}

impl WebElement {
    /// Send keys to the element
    pub async fn send_keys(&self, keys: &str) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/element/{}/value", self.session.session_id, self.element_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({ "text": keys })).await?;
        Ok(())
    }

    /// Click the element
    pub async fn click(&self) -> WebDriverResult<()> {
        let endpoint = format!("/session/{}/element/{}/click", self.session.session_id, self.element_id);
        self.session.client.post::<()>(&endpoint, &serde_json::json!({})).await?;
        Ok(())
    }

    /// Get the text content of the element
    pub async fn text(&self) -> WebDriverResult<String> {
        let endpoint = format!("/session/{}/element/{}/text", self.session.session_id, self.element_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_str().unwrap_or("").to_string())
    }

    /// Get an attribute of the element
    pub async fn attribute(&self, name: &str) -> WebDriverResult<String> {
        let endpoint = format!("/session/{}/element/{}/attribute/{}", self.session.session_id, self.element_id, name);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_str().unwrap_or("").to_string())
    }

    /// Check if the element is displayed
    pub async fn is_displayed(&self) -> WebDriverResult<bool> {
        let endpoint = format!("/session/{}/element/{}/displayed", self.session.session_id, self.element_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_bool().unwrap_or(false))
    }

    /// Check if the element is enabled
    pub async fn is_enabled(&self) -> WebDriverResult<bool> {
        let endpoint = format!("/session/{}/element/{}/enabled", self.session.session_id, self.element_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_bool().unwrap_or(false))
    }

    /// Check if the element is selected
    pub async fn is_selected(&self) -> WebDriverResult<bool> {
        let endpoint = format!("/session/{}/element/{}/selected", self.session.session_id, self.element_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_bool().unwrap_or(false))
    }

    /// Get the rect (position and size) of the element
    pub async fn rect(&self) -> WebDriverResult<ElementRect> {
        let endpoint = format!("/session/{}/element/{}/rect", self.session.session_id, self.element_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(ElementRect::from_response(&response["value"]))
    }

    /// Get the tag name of the element
    pub async fn tag_name(&self) -> WebDriverResult<String> {
        let endpoint = format!("/session/{}/element/{}/name", self.session.session_id, self.element_id);
        let response: serde_json::Value = self.session.client.get(&endpoint).await?;
        Ok(response["value"].as_str().unwrap_or("").to_string())
    }
}

/// Represents the rect (position and size) of an element
#[derive(Debug, Clone)]
pub struct ElementRect {
    /// X coordinate of the element
    pub x: f64,
    /// Y coordinate of the element
    pub y: f64,
    /// Width of the element
    pub width: f64,
    /// Height of the element
    pub height: f64,
}

impl ElementRect {
    fn from_response(value: &serde_json::Value) -> Self {
        Self {
            x: value["x"].as_f64().unwrap_or(0.0),
            y: value["y"].as_f64().unwrap_or(0.0),
            width: value["width"].as_f64().unwrap_or(0.0),
            height: value["height"].as_f64().unwrap_or(0.0),
        }
    }

    /// Check if the element is visible (has positive dimensions and is on screen)
    pub fn is_visible(&self) -> bool {
        self.width > 0.0 && self.height > 0.0 && self.x >= 0.0 && self.y >= 0.0
    }
}
