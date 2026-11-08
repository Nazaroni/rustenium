//! Rustenium Web API Prelude
//!
//! This module re-exports commonly used types and traits for convenient importing.

pub use crate::browser::{
    Browser, BrowserCapabilities, ChromeCapabilities, ChromeOptions,
    FirefoxCapabilities, FirefoxOptions, SafariCapabilities, SafariOptions,
    EdgeCapabilities, EdgeOptions,
};
pub use crate::element::{WebElement, ElementRect};
pub use crate::launcher::{DriverLauncher, LauncherError};
pub use crate::{WebDriver, WebDriverConfig};
