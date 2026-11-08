// Example: Waits and Synchronization with Rustenium Web
//
// This example demonstrates:
// - Using wait_for_element for implicit element waiting
// - Custom waits with wait_until
// - Waiting for visibility

use rustenium_web::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = WebDriver::new_chrome().await?;

    // Navigate to a page with dynamic content
    driver
        .goto("https://www.w3schools.com/js/tryit.asp?filename=tryjs_timing1")
        .await?;

    // Wait for the iframe containing the demo
    let iframe = driver
        .wait_for_element("iframe#iframeResult", Duration::from_secs(10))
        .await?;

    if let Some(element) = iframe {
        // Switch to the iframe (if needed)
        println!("Found iframe");

        // Wait for element visibility
        let button = driver
            .wait_until_visible("button", Duration::from_secs(10))
            .await?;

        if let Some(element) = button {
            println!("Button is now visible");

            // Check element is displayed
            if element.is_displayed().await? {
                println!("Button is displayed");

                // Get button text before clicking
                let text = element.text().await?;
                println!("Button text: {}", text);

                // Click the button
                element.click().await?;
                println!("Clicked the button");

                // Wait for dynamic content to appear
                tokio::time::sleep(Duration::from_secs(1)).await;

                // Verify the result
                let p = driver.find_element("p").await?;
                println!("Result text: {}", p.text().await?);
            }
        }
    }

    // Clean up
    driver.quit().await?;

    Ok(())
}
