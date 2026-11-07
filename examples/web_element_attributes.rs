// Example: Element Attributes and Properties with Rustenium Web
//
// This example demonstrates:
// - Getting element text, attributes, and properties
// - Checking element state (displayed, enabled, selected)
// - Getting element position and size

use rustenium_web::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = WebDriver::new_chrome().await?;

    // Navigate to a page with various elements
    driver
        .goto("https://www.w3schools.com/html/html_attributes.asp")
        .await?;

    // Wait for content to load
    driver.wait_for_element("h1", Duration::from_secs(10)).await?;

    // Get element text
    let h1 = driver.find_element("h1").await?;
    println!("Heading text: {}", h1.text().await?);

    // Get element attribute
    let id = h1.attribute("id").await?;
    println!("Heading ID: {:?}", id);

    // Get element tag name using the method
    let tag_name = h1.tag_name().await?;
    println!("Tag name: {:?}", tag_name);

    // Find an input element
    let inputs = driver.find_elements("input").await?;
    if !inputs.is_empty() {
        let input = &inputs[0];

        // Check if element is displayed
        println!("Input is displayed: {}", input.is_displayed().await?);

        // Check if element is enabled
        println!("Input is enabled: {}", input.is_enabled().await?);

        // Get element rect (position and size)
        if let Ok(rect) = input.rect().await {
            println!("Input position: x={}, y={}", rect.x, rect.y);
            println!("Input size: width={}, height={}", rect.width, rect.height);
        }

        // Send keys to input
        input.send_keys("Test input").await?;
        println!("Typed into input");

        // Get value attribute
        let value = input.attribute("value").await?;
        println!("Input value: {:?}", value);
    }

    // Clean up
    driver.quit().await?;

    Ok(())
}
