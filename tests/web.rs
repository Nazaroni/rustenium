use rustenium::common::capabilities::ChromeCapabilities;
use rustenium::webdriver::client::WebDriverHttpClient;
use rustenium::webdriver::session::WebDriverSession;
use rustenium::web::browser::BrowserCapabilities;
use rustenium::web::element::WebElement;

#[tokio::test]
async fn test_rust_site_search() {
    let client = WebDriverHttpClient::new("http://localhost:9515");
    let capabilities = BrowserCapabilities::Chrome(ChromeCapabilities::new()).to_value();
    let session = WebDriverSession::create(client.clone(), capabilities).await.expect("session");

    // Navigate to Rust website
    let endpoint = format!("session/{}/url", session.session_id);
    let _ = client.post(&endpoint, &serde_json::json!({"url": "https://www.rust-lang.org/"})).await.expect("navigate");

    // Find the search input element (example: by CSS selector)
    let find_endpoint = format!("session/{}/element", session.session_id);
    let params = serde_json::json!({"using": "css selector", "value": "input[type='search']"});
    let resp = client.post(&find_endpoint, &params).await.expect("find element");
    let element_id = resp["value"]["element-6066-11e4-a52e-4f735466cecf"].as_str().unwrap_or("").to_string();
    let element = WebElement { element_id, session };

    // Send keys to the element
    element.send_keys("rust").await.expect("send keys");
}
