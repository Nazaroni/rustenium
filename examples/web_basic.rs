// Example: Basic Web Automation with Rustenium Web
//
// This example demonstrates:
// - Creating a WebDriver with custom configuration
// - Navigation (go to URL, back, forward, refresh)
// - Getting page attributes (title, current URL)
// - Finding elements by CSS selector

use rustenium_web::{WebDriverConfig, WebDriver};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a custom WebDriver configuration
    let config = WebDriverConfig::new()
        .with_base_url("http://localhost:4444")
        .with_port(9515);

    // Create a Chrome WebDriver with the custom configuration
    let driver = WebDriver::new(&config).await?;

    // Navigate to a page
    println!("Going to example.com...");
    driver.goto("https://example.com").await?;

    // Get page title
    let title = driver.title().await?;
    println!("Title: {}", title);

    // Get current URL (from session)
    let url = format!("{}/session/{}", config.base_url, driver.session_id());
    println!("Current URL: {}", url);

    // Find the main heading
    let h1 = driver.find_element("h1").await?;
    println!("Main heading text: {}", h1.text().await?);

    // Find all links on the page
    let links = driver.find_elements("a").await?;
    println!("Found {} links", links.len());

    // Navigate to another page
    driver.goto("https://example.com/about").await?;
    println!("Navigated to about page");
    println!("Title: {}", driver.title().await?);

    // Go back
    driver.back().await?;
    println!("Went back");
    println!("Title: {}", driver.title().await?);

    // Go forward
    driver.forward().await?;
    println!("Went forward");

    // Refresh the page
    driver.refresh().await?;
    println!("Refreshed");

    // Clean up
    driver.quit().await?;

    Ok(())
}
