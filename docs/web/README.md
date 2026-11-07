# Web Automation

The `web` crate provides a high-level API for browser automation, similar to
Selenium WebDriver.

## Features

- **Browser Control**: Launch and manage browser instances (Chrome, Firefox,
  Safari, Edge).
- **Element Interaction**: Find elements, click, type, extract text, get attributes.
- **Navigation**: Go to URLs, navigate back/forward, refresh.
- **Waits**: Wait for elements to be present, visible, or clickable.
- **Configurable WebDriver**: Customizable driver configuration with headless mode.

## Quick Start

```rust
use rustenium_web::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Chrome WebDriver with default settings (headless mode)
    let driver = WebDriver::new_chrome().await?;
    
    // Navigate to a URL
    driver.goto("https://example.com").await?;
    
    // Get the page title
    let title = driver.title().await?;
    println!("Title: {}", title);
    
    // Find an element and interact with it
    let link = driver.find_element("a").await?;
    let text = link.text().await?;
    println!("Link text: {}", text);
    
    driver.quit().await?;
    Ok(())
}
```

## Advanced Usage

### Custom Configuration

```rust
use rustenium_web::{WebDriver, WebDriverConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom configuration
    let config = WebDriverConfig::new()
        .with_base_url("http://localhost:4444")
        .with_port(9515)
        .with_headless(false); // Run with visible browser
    
    let driver = WebDriver::new(&config).await?;
    
    driver.goto("https://example.com").await?;
    // ... your code here
    
    driver.quit().await?;
    Ok(())
}
```

### Using Waits

```rust
use rustenium_web::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = WebDriver::new_chrome().await?;
    
    // Wait for an element to be present
    let element = driver.wait_for_element("#my-element", Duration::from_secs(10)).await?;
    
    // Wait for an element to be visible
    let button = driver.wait_until_visible("button.submit", Duration::from_secs(5)).await?;
    
    button.click().await?;
    
    driver.quit().await?;
    Ok(())
}
```

### All Browsers

```rust
use rustenium_web::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use Firefox
    let driver = WebDriver::new_firefox().await?;
    
    driver.goto("https://example.com").await?;
    
    // Use back/forward/refresh
    driver.back().await?;
    driver.forward().await?;
    driver.refresh().await?;
    
    driver.quit().await?;
    Ok(())
}
```

### Using DriverLauncher (Advanced)

For advanced use cases where you need direct control over the WebDriver server
process, you can use `DriverLauncher` to launch and configure the driver:

```rust
use rustenium_web::launcher::DriverLauncher;
use std::process::Child;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a launcher with custom settings
    let mut launcher = DriverLauncher::new()
        .with_driver_path("/path/to/chromedriver")
        .with_port(9515)
        .with_headless(true);
    
    // Start the driver process
    let child: Child = launcher.start()?;
    
    // The driver is now running... use it with your WebDriver client
    
    Ok(())
}
```

#### DriverLauncher Features:

- **Automatic Port Management**: Checks if port is in use before starting
- **Startup Timeout**: Waits for driver to be ready (default 5 seconds)
- **Process Cleanup**: Kills the process if startup times out
- **Headless Mode Support**: Configurable headless/visible mode
