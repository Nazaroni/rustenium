# Mobile Automation Example

## iOS Example

```rust
use rustenium::mobile::ios::{IosCapabilities, create_ios_session};
use rustenium::webdriver::client::WebDriverHttpClient;

#[tokio::main]
async fn main() {
    let client = WebDriverHttpClient::new("http://localhost:4723/wd/hub");
    let caps = IosCapabilities::new("iPhone Simulator", "/path/to/your.app");
    let session = create_ios_session(client, caps).await.unwrap();
    println!("iOS session started: {}", session.session_id);
}
```

## Android Example

```rust
use rustenium::mobile::android::{AndroidCapabilities, create_android_session};
use rustenium::webdriver::client::WebDriverHttpClient;

#[tokio::main]
async fn main() {
    let client = WebDriverHttpClient::new("http://localhost:4723/wd/hub");
    let caps = AndroidCapabilities::new("Android Emulator", "/path/to/your.apk");
    let session = create_android_session(client, caps).await.unwrap();
    println!("Android session started: {}", session.session_id);
}
```

> **Note:**
>
> - Appium server must be running and accessible at the given URL.
> - The device/emulator and app paths must be correct.
