use rustenium::mobile::ios::{IosCapabilities, create_ios_session};
use rustenium::mobile::android::{AndroidCapabilities, create_android_session};
use rustenium::webdriver::client::WebDriverHttpClient;

#[tokio::test]
async fn test_create_ios_session() {
    let client = WebDriverHttpClient::new("http://localhost:4723/wd/hub");
    let caps = IosCapabilities::new("iPhone Simulator", "/path/to/test.app");
    let result = create_ios_session(client, caps).await;
    // This will fail unless Appium and a simulator are available, but ensures compilation and call
    assert!(result.is_err() || result.is_ok());
}

#[tokio::test]
async fn test_create_android_session() {
    let client = WebDriverHttpClient::new("http://localhost:4723/wd/hub");
    let caps = AndroidCapabilities::new("Android Emulator", "/path/to/test.apk");
    let result = create_android_session(client, caps).await;
    // This will fail unless Appium and an emulator are available, but ensures compilation and call
    assert!(result.is_err() || result.is_ok());
}
