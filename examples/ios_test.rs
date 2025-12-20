use rustenium::mobile::ios::{IosCapabilities, create_ios_session};
use rustenium::webdriver::client::WebDriverHttpClient;

#[tokio::main]
async fn main() {
    // Example: assumes Rustenium-mobile running at 4723 and .app file path is correct
    let client = WebDriverHttpClient::new("http://localhost:4723/wd/hub");
    let caps = IosCapabilities::new("iPhone Simulator", "/path/to/your.app");
    let session = create_ios_session(client.clone(), caps)
        .await
        .expect("failed to create iOS session");
    // Example: implement element search, click, send_keys as needed
    println!("iOS session started: {}", session.session_id);
    // session.delete().await.ok();
}
