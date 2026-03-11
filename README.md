> 🚧 **WARNING: Rustenium is under active development and not yet production-ready.**
>
> Expect breaking changes, incomplete features, and unstable behavior. **Do not use this project in production systems yet.**

<div align="center">
  <h1>🦀 Rustenium 🦀</h1>
  <p>
    <strong>A Rust-native, async-first, multicore end-to-end automation framework
    for Web, Android, and iOS.</strong>
  </p>
  <p>
    <a href="https://github.com/user/rustenium/actions/workflows/ci.yml">
      <img src="https://img.shields.io/github/actions/workflow/status/user/rustenium/ci.yml?branch=main&style=for-the-badge" alt="Build Status">
    </a>
    <a href="https://crates.io/crates/rustenium">
      <img src="https://img.shields.io/crates/v/rustenium.svg?style=for-the-badge" alt="Crates.io">
    </a>
    <a href="https://opensource.org/licenses/MIT">
      <img src="https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge" alt="License: MIT">
    </a>
  </p>
</div>

---

**Rustenium** is inspired by Selenium, but completely rewritten in Rust to
provide a unified, high-performance, and type-safe API for browser and mobile
automation. It comes with its own BDD test runner, assertion library, and
reporting tools, offering a complete end-to-end testing solution.

## ✨ Key Features

- ⚡️ **Async-First & Multicore:** Built from the ground up with `async` to run
  tests in parallel, leveraging multiple cores for maximum speed.
- 🌐 **Web Automation:** Drives modern browsers like Chrome, Firefox, and Safari
  using the W3C WebDriver standard.
- 📱 **Mobile Automation:** Supports both Android (via UiAutomator2) and iOS
  (via XCUITest) for native and hybrid app testing.
- 🧩 **Complete Toolchain:**
  - **BDD Runner:** Write tests in Gherkin syntax.
  - **Assertion Library:** Fluent and expressive assertions.
  - **Reporter:** Generate beautiful, detailed test reports.
- 🔒 **Type-Safe API:** Catch errors at compile time, not runtime, thanks to
  Rust's powerful type system.

## 🚀 Getting Started

### 1. Prerequisites

Ensure you have the necessary drivers installed and available in your system's
`PATH`.

- **For Web:** `chromedriver`, `geckodriver`, etc.
- **For Mobile:** A working Rustenium-mobile setup.

Our helper scripts in the `/scripts` directory can assist you.

### 2. Add to Your Project

Add Rustenium and its components to your `Cargo.toml`:

```toml
[dependencies]
rustenium-core = "0.1.0"
rustenium-web = "0.1.0"
rustenium-mobile = "0.1.0"
# ... and other crates you need
```

### 3. Write Your First Test

Check out the examples in the `📁 examples/` directory to see how to write
tests for web, Android, and iOS.

```rust
// Example web test (see examples/ for more)
use rustenium_web::WebDriver;

#[tokio::test]
async fn my_web_test() {
    let mut driver = WebDriver::new_chrome().await.unwrap();
    driver.goto("https://www.google.com").await.unwrap();
    assert_eq!(driver.title().await.unwrap(), "Google");
    driver.quit().await.unwrap();
}
```

---

## 📂 Repository Structure

This project is a Cargo workspace, making it modular and scalable.

- `📁 crates/`: Home to all the individual packages (crates).
  - `⚙️ core/`: Shared logic, traits, and WebDriver/Rustenium-mobile protocol
    implementation.
  - `🌐 web/`: Web browser automation features.
  - `📱 mobile/`: Unified API for mobile automation.
  - `🍎 ios/`: iOS-specific implementation.
  - `🤖 android/`: Android-specific implementation.
  - `🥒 bdd/`: Business Driven Development (BDD) runner.
  - `✅ assertions/`: Expressive assertion library.
  - `📊 reporter/`: Test report generation tool.
  - `⌨️ cli/`: Command-line interface for the test runner.
- `📁 examples/`: Example tests and usage scenarios.
- `📁 tests/`: Integration tests for the Rustenium framework itself.
- `📁 scripts/`: Helper scripts for setup and driver management.
- `📁 .github/`: CI/CD workflows for automated testing and releases.
- `📁 docs/`: In-depth documentation and API references.
- `📄 Cargo.toml`: The workspace definition file.

---

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md)
to get started.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md)
file for details.
