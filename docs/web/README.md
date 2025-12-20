# Web Automation

The `web` crate provides a high-level API for browser automation, similar to
Selenium WebDriver.

## Features

- **Browser Control**: Launch and manage browser instances (Chrome, Firefox,
  Safari, Edge).
- **Element Interaction**: Find elements, click, type, and extract text.
- **Navigation**: Go to URLs, navigate back/forward, refresh.
- **Waits**: Wait for elements to be present, visible, or clickable.

## Example

```rust
use rustenium_web::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = WebDriver::new().await?;
    driver.get("https://example.com").await?;
    let title = driver.title().await?;
    println!("Title: {}", title);
    driver.quit().await?;
    Ok(())
}
```
