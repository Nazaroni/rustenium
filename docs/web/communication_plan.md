# WebDriver Communication Implementation Plan

## Current State (as of commit `b4954bc`)
| Area | Status |
|------|--------|
| **Browser Capabilities** | Only Chrome support via `ChromeCapabilities` in `browser.rs`. No generic enum for Firefox/Safari/Edge. |
| **WebDriver Session** | Basic session creation/deletion implemented in `session.rs`. No handling for remote grid or hub. |
| **WebDriver API** | `new`, `goto`, `title`, `find_element`, `quit`. No navigation helpers, no wait logic. |
| **Element API** | `send_keys`, `click`. No text extraction, attribute queries, visibility checks. |
| **Error Handling** | Basic `WebDriverError` enum; does not translate W3C error codes into domain-specific errors. |
| **Driver Launch** | No automatic launch of `chromedriver`/`geckodriver`. User must start driver manually. |
| **Documentation** | README examples refer to `WebDriver::new_chrome()` which does not exist. |
| **Testing** | Minimal integration tests; no end‑to‑end communication test with a real driver. |

---
## What We Need to Add / Improve
1. **Unified Browser Capability System**
   * Define a `Browser` enum with variants: Chrome, Firefox, Safari, Edge.
   * Create dedicated capability structs for each browser (e.g., `ChromeOptions`, `FirefoxOptions`).
   * Implement `Capabilities` trait for each struct.
2. **Driver Launch Helper**
   * Provide a `DriverLauncher` that can spawn the appropriate driver binary.
   * Detect if a driver is already running on the requested port; otherwise start it.
   * Allow configuration via environment variables or a config struct.
3. **Convenience Constructors**
   * Add `WebDriver::new_chrome()`, `WebDriver::new_firefox()`, etc.
   * These constructors should start the driver (if needed) and return a ready‑to‑use `WebDriver` instance.
4. **Navigation & Wait Utilities**
   * Add methods: `back`, `forward`, `refresh`.
   * Implement a generic `wait_until<T>` helper that polls a closure until it returns true or times out.
   * Provide convenience waits: `wait_for_element`, `wait_until_visible` etc.
5. **Expanded Element API**
   * Add `text()`, `attribute(name)`, `is_displayed()`, `get_rect()`.
   * Support CSS selector, XPath, ID, name locators via an enum `By`.
6. **Error Mapping**
   * Map W3C error codes (e.g., `no such element`, `session not created`) to meaningful Rust errors.
   * Provide a `Result<T, WebDriverError>` that can be converted to custom error types.
7. **Configuration & Options**
   * Allow the user to specify base URL, port, driver binary path, and headless flag.
   * Expose a `WebDriverConfig` struct that can be passed to constructors.
8. **Documentation & Examples**
   * Update README and `docs/web/README.md` with correct examples.
   * Add a `prelude.rs` re‑export module for common imports.
9. **Testing**
   * Write integration tests that launch a real `chromedriver`, perform navigation, element interaction, and cleanup.
   * Use the helper scripts in `/scripts` to download drivers for CI.
10. **CI Integration**
    * Update GitHub Actions to install drivers on Linux, macOS, Windows.
    * Add matrix jobs for different browsers.

---
## Milestone Roadmap
| Milestone | Deliverables | Owner | Target |
|-----------|--------------|-------|--------|
| **M1: Core API** | Implement `WebDriver::new_chrome()`, basic session creation, navigation helpers. | @dev1 | 2 days |
| **M2: Capabilities** | Browser enum, capability structs, `Capabilities` impls. | @dev2 | 1 day |
| **M3: Driver Launcher** | Process spawning, port detection, environment config. | @dev1 | 2 days |
| **M4: Element API** | Text, attribute, visibility methods. | @dev2 | 1 day |
| **M5: Waits & Errors** | Generic wait, error mapping. | @dev1 | 1 day |
| **M6: Documentation & Tests** | Updated README, prelude, integration tests. | @dev3 | 1 day |
| **M7: CI & Matrix** | GitHub Actions matrix, driver installation scripts. | @dev3 | 1 day |

---
## Risks & Mitigations
| Risk | Impact | Mitigation |
|------|--------|------------|
| Driver binaries not available for all OSes | Medium | Use the existing `/scripts` to download. |
| Port conflicts when launching drivers | High | Detect used ports; allow user override via config. |
| API changes in W3C spec | Low | Wrap responses; provide abstraction for future changes. |

---
## Next Steps
1. Create `Browser` enum and capability structs.
2. Implement `WebDriver::new_chrome()` that uses `DriverLauncher`.
3. Add navigation helpers and basic wait logic.
4. Expand element API.
5. Write integration tests.
6. Update docs and CI.

---
*Prepared by the Rustenium Core Team.*