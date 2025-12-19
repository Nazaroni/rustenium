# Extensions Usage Examples

## BiDi (BiDirectional) Stub

```rust
use rustenium::extensions::bidi::BiDiClient;
use rustenium::webdriver::client::WebDriverHttpClient;

#[tokio::main]
async fn main() {
    let client = WebDriverHttpClient::new("http://localhost:9515");
    let bidi = BiDiClient::new(client);
    let resp = bidi.send_command("example.method", serde_json::json!({"foo": "bar"})).await.unwrap();
    println!("BiDi response: {resp:?}");
}
```

## Grid Session Creation

```rust
use rustenium::extensions::grid::create_grid_session;
use rustenium::common::capabilities::ChromeCapabilities;
use rustenium::web::browser::BrowserCapabilities;

#[tokio::main]
async fn main() {
    let capabilities = BrowserCapabilities::Chrome(ChromeCapabilities::new()).to_value();
    let session = create_grid_session("http://localhost:4444/wd/hub", capabilities).await.unwrap();
    println!("Grid session started: {}", session.session_id);
}
```

## Screenshot

```rust
use rustenium::webdriver::client::WebDriverHttpClient;
use rustenium::webdriver::session::WebDriverSession;
use rustenium::common::capabilities::ChromeCapabilities;
use rustenium::web::browser::BrowserCapabilities;

#[tokio::main]
async fn main() {
    let client = WebDriverHttpClient::new("http://localhost:9515");
    let capabilities = BrowserCapabilities::Chrome(ChromeCapabilities::new()).to_value();
    let session = WebDriverSession::create(client.clone(), capabilities).await.unwrap();
    let png_bytes = session.screenshot().await.unwrap();
    std::fs::write("screenshot.png", &png_bytes).unwrap();
    println!("Screenshot saved");
}
```

## Logging

```rust
use rustenium::webdriver::client::WebDriverHttpClient;
use rustenium::webdriver::session::WebDriverSession;
use rustenium::common::capabilities::ChromeCapabilities;
use rustenium::web::browser::BrowserCapabilities;

#[tokio::main]
async fn main() {
    let client = WebDriverHttpClient::new("http://localhost:9515");
    let capabilities = BrowserCapabilities::Chrome(ChromeCapabilities::new()).to_value();
    let session = WebDriverSession::create(client.clone(), capabilities).await.unwrap();
    let logs = session.get_logs("browser").await.unwrap();
    println!("Browser logs: {logs:?}");
}
```

## Page Object Model Macro

```rust
use rustenium::define_component;

define_component!(LoginPage, username: "#username", password: "#password", submit: "#submit");

fn main() {
    let selectors = LoginPage::selectors();
    println!("Selectors: {selectors:?}");
}
```
