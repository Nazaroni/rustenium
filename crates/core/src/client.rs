use reqwest::{header::CONTENT_TYPE, Client, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebDriverError {
    #[error("Request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("WebDriver error: {0}")]
    WebDriver(String),
}

pub type WebDriverResult<T> = Result<T, WebDriverError>;

#[derive(Clone, Debug)]
pub struct WebDriverHttpClient {
    client: Client,
    base_url: String,
}

impl WebDriverHttpClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn post<T: DeserializeOwned>(&self, endpoint: &str, body: &Value) -> WebDriverResult<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.post(&url).header(CONTENT_TYPE, "application/json").json(body).send().await?;
        self.handle_response(response).await
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> WebDriverResult<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;
        self.handle_response(response).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, endpoint: &str) -> WebDriverResult<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        let response = self.client.delete(&url).send().await?;
        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> WebDriverResult<T> {
        if response.status().is_success() {
            let body = response.json::<T>().await?;
            Ok(body)
        } else {
            let error_body: Value = response.json().await?;
            Err(WebDriverError::WebDriver(error_body.to_string()))
        }
    }
}
