// Example: Google Search with Rustenium Web
//
// This example demonstrates:
// - Creating a WebDriver instance
// - Navigating to a URL
// - Finding elements by CSS selector
// - Interacting with elements (sending keys, clicking)
// - Getting element text
// - Using waits

use rustenium_web::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Chrome WebDriver (headless by default)
    let driver = WebDriver::new_chrome().await?;

    // Navigate to Google
    driver.goto("https://www.google.com").await?;
    println!("Navigated to Google");

    // Wait for the search box to be present
    let search_box = driver
        .wait_for_element("textarea[name='q']", Duration::from_secs(10))
        .await?;

    if let Some(element) = search_box {
        // Type in the search box
        element
            .send_keys("Rust programming language")
            .await?;
        println!("Typed search query");

        // Wait a moment then press Enter
        tokio::time::sleep(Duration::from_millis(500)).await;
        element.send_keys("\u{0d}").await?;
        println!("Pressed Enter");

        // Wait for results
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Get the page title
        let title = driver.title().await?;
        println!("Page title: {}", title);

        // Find search results
        let results = driver.find_elements("h3").await?;
        println!("Found {} search result headings", results.len());

        // Print first 5 result titles
        for (i, result) in results.iter().take(5).enumerate() {
            if let Ok(text) = result.text().await {
                println!("{i}: {}", text);
            }
        }
    }

    // Wait before quitting
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Clean up
    driver.quit().await?;
    println!("Browser closed");

    Ok(())
}
