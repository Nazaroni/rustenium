# Android Communication Plan (Native & WebView)

## Overview
This document outlines the architecture, key components, and implementation roadmap for enabling robust communication with Android devices from Rustenium. The goal is to provide a **native** automation layer that works with Rustenium’s custom WebDriver server (W3C‑compliant) and a seamless **WebView** integration that allows switching between native and web contexts.

The plan covers:
1. Core data structures – capabilities, sessions, context handling.
2. HTTP client interaction – request/response flow.
3. Error mapping & retry logic.
4. WebView support – context discovery, switch, element querying.
5. Testing strategy and CI integration.
6. Documentation and example usage.

---
## 1. Core Data Structures
### 1.1 AndroidCapabilities
* Fields:
  * `device_name: String`
  * `platform_version: Option<String>`
  * `app_path: Option<PathBuf>` – path to APK for native app testing.
  * `automation_name: Option<String>` – e.g., "UiAutomator2".
  * `browser_name: Option<String>` – e.g., "Chrome" or "WebView".
  * `app_package: Option<String>` – package name for WebView contexts.
  * `app_activity: Option<String>` – activity to launch.
  * `no_reset: bool`
* Implements `Capabilities` trait → `to_value()` that serializes into the JSON format expected by Rustenium’s custom WebDriver server.

### 1.2 AndroidSession
* Wraps `WebDriverSession` from `rustenium-core`.
* Provides convenience methods:
  * `new(client: WebDriverHttpClient, caps: AndroidCapabilities) -> Result<Self>` – POST /session.
  * `contexts(&self) -> Result<Vec<String>>` – GET /session/{id}/contexts.
  * `switch_context(&self, context_name: &str) -> Result<()>` – POST /session/{id}/context.
  * `quit(self)` – DELETE session.

---
## 2. HTTP Client Interaction
* Reuse `WebDriverHttpClient` from core.
* All requests are async and use the W3C WebDriver JSON protocol.
* Base URL: typically `http://localhost:{port}/wd/hub` where `{port}` is Rustenium’s custom WebDriver server's listening port.
* Helper functions for building endpoints:
  * `/session`
  * `/session/{id}/{command}`

---
## 3. Error Mapping & Retry Logic
* Translate Rustenium’s custom WebDriver server error codes (e.g., `noSuchElement`, `invalidArgument`) into a custom enum `AndroidError` that implements `std::error::Error`.
* Provide a retry wrapper for transient errors such as network timeouts or session drops.
* Use exponential back‑off with configurable max retries.

---
## 4. WebView Support
### 4.1 Context Discovery
* After creating a session, call `contexts()` to get list of contexts: e.g., `NATIVE_APP`, `WEBVIEW_1`.
* Provide helper `native_context()` and `webview_contexts() -> Vec<String>`.

### 4.2 Switching Contexts
* `switch_to_webview(name)` – POST /session/{id}/context with `{"name": "<ctx>"}`.
* After switching, the same WebDriver commands can be used to locate elements inside the webview.

### 4.3 Element Interaction in WebView
* Re‑export `WebElement` from core; ensure it works for both native and web contexts.
* Provide convenience methods like `find_element_by_css`, `find_element_by_xpath` that internally send appropriate locator strategy.

---
## 5. Testing Strategy
1. **Unit Tests** – Verify capability serialization, context parsing, error mapping.
2. **Integration Tests** – Spin up a local Rustenium’s custom WebDriver server server (via Docker) and run tests against an emulator or test APK.
3. **WebView Tests** – Load a simple webview inside a native app (e.g., Android WebView component) and perform element queries.
4. **CI Matrix** – Run on Linux, macOS, Windows with Rustenium’s custom WebDriver server Docker images.

---
## 6. Documentation & Examples
* Update `docs/mobile/README.md` to include:
  * Example for native app session creation.
  * Switching to WebView context and interacting with elements.
  * Sample test code in `examples/android_webview_test.rs`.
* Provide a prelude module: `pub mod prelude { pub use crate::{AndroidCapabilities, AndroidSession}; }`.

---
## Milestone Roadmap
| # | Deliverable | Owner | Target |
|---|-------------|-------|--------|
| 1 | Define `AndroidCapabilities`, implement `Capabilities` trait | @dev1 | 2 days |
| 2 | Implement `AndroidSession::new` and context methods | @dev1 | 2 days |
| 3 | Error mapping & retry logic | @dev2 | 1 day |
| 4 | WebView context handling helpers | @dev1 | 1 day |
| 5 | Unit tests for capabilities & session creation | @dev2 | 1 day |
| 6 | Integration test against Rustenium’s custom WebDriver server Docker image | @dev3 | 2 days |
| 7 | Update docs and examples | @dev1 | 1 day |
| 8 | CI matrix configuration | @dev3 | 1 day |

---
## Risks & Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| Rustenium’s custom WebDriver server server version changes | Medium | Keep serialization flexible; use serde's `#[serde(flatten)]` for future fields. |
| WebView context name variations | Low | Provide helper to filter contexts by prefix `WEBVIEW_`. |
| Emulator availability on CI | High | Use Docker image with pre‑installed emulator or run tests conditionally if device present. |

---
## Next Steps
1. Create the `android` crate (or fill in the existing placeholder).
2. Add the capability struct and session implementation.
3. Wire up context switching logic.
4. Write unit tests.
5. Draft example usage.
6. Commit, tag, and update CI.
