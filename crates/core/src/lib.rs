//! # Rustenium Core Library
//!
//! `rustenium_core` is a collection of utilities used by the Rustenium project.
//! It provides common functionality, types, and utilities that are shared across
//! different components of the Rustenium ecosystem.
//! This library is designed to be a foundational layer that other Rustenium
//! libraries can depend on, making it easier to maintain and reuse code across the project.

pub mod capabilities;
pub mod client;
pub mod common;
pub mod session;

/// Say Hallo from the core library.
///
/// # Examples
///
/// ```
/// let result = rustenium_core::hello_from_rustenium_core();
/// assert_eq!(result, "Hello from rustenium_core!!!");
///
/// ```
pub fn hello_from_rustenium_core() -> String {
    "Hello from rustenium_core!!!".to_string()
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rustenium_core::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*; // import all code for testing into the tests scope

    #[test]
    fn it_works() {
        let result = hello_from_rustenium_core();
        assert_eq!(result, "Hello from rustenium_core!!!");
    }
}
