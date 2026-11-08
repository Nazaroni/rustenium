# iOS Implementation Plan

## Overview
Add full support for iOS automation via XCUITest, matching Android feature parity while keeping a clean API that plugs into `rustenium-core`.

| Phase | Focus | Deliverables | Target |
|-------|-------|--------------|--------|
| **1 – Core Session & Capabilities** | Build the foundation: capability struct, session lifecycle, basic element finding. | • `IosCapabilities` type<br>• `create_ios_session()` helper<br>• XCUITest session creation (`POST /session`) <br>• Basic `WebDriverSession` wrappers for iOS <br>• Unit tests covering happy‑path session init | 2 days |
| **2 – Advanced Features & Testing** | Expand API surface, error handling, touch actions, device control, CI integration. | • Element interaction methods (`click`, `send_keys`, etc.)<br>• Touch action helpers (tap, swipe, pinch)<br>• Device‑level commands (rotate, network toggle)<br>• Error mapping & retry logic<br>• Integration tests against a local XCUITest server or simulator<br>• Documentation & examples | 3 days |

### Phase 1 – Core Session & Capabilities
| Milestone | Description |
|-----------|-------------|
| **M1.1 – Capability Struct** | Define `IosCapabilities` mirroring the fields used in Android (`device_name`, `platform_version`, `app_path`, etc.) and implement `Capabilities` trait to serialize into the XCUITest JSON format. |
| **M1.2 – Session Creation** | Implement `create_ios_session(client, caps)` that POSTs `/session` to a local Rustenium‑mobile server (default `http://localhost:4723/wd/hub`). Handle response, store session ID, and expose it via `WebDriverSession`. |
| **M1.3 – Basic Element API** | Wire up minimal element finding (`find_element_by_xpath`, etc.) through the XCUITest JSON‑Wire commands. These should be thin wrappers around `rustenium-core::WebDriverSession`. |
| **M1.4 – Unit Tests** | Add tests in `tests/mobile.rs` that mock a local server (or use a real one if available) to verify session creation and element finding. |

> **Risk & Mitigation:**
> • *No XCUITest server running* → Provide a test harness that spins up the official Appium‑based Rustenium‑mobile image in Docker for CI.
> • *Capability mismatch* → Validate against the official XCUITest capabilities spec and use Serde’s `flatten` to allow future extensions.

### Phase 2 – Advanced Features & Testing
| Milestone | Description |
|-----------|-------------|
| **M2.1 – Element Interaction** | Implement `click`, `send_keys`, `get_text`, etc., delegating to XCUITest commands. Ensure they return appropriate Rust types (`String`, `bool`). |
| **M2.2 – Touch Actions** | Add a `TouchAction` builder that supports tap, swipe, pinch, and multi‑touch gestures, mirroring the WebDriver W3C actions spec for mobile. |
| **M2.3 – Device Control** | Expose methods to rotate screen, toggle network, install/uninstall apps via XCUITest custom commands (if supported). |
| **M2.4 – Error Mapping & Retry Logic** | Translate XCUITest error codes (`noSuchElement`, `sessionNotCreated`) into a `IosError` enum that implements `std::error::Error`. Add exponential‑backoff retry for transient network failures. |
| **M2.5 – Integration Tests** | Spin up an actual iOS simulator (or use a prebuilt Docker image) and run end‑to‑end tests covering session creation, element interaction, touch actions, and device control. |
| **M2.6 – Documentation & Examples** | Update `docs/mobile/README.md` to include usage examples, add a new `examples/ios_test.rs`, and generate Rust docs for the iOS crate. |
| **M2.7 – CI Pipeline** | Add a GitHub Actions job that builds the iOS crate on macOS (to run simulator) or uses a Docker image with XCUITest server. |

> **Risk & Mitigation:**
> • *Simulator availability* → Use `xcrun simctl` in CI to create and boot simulators, fall back to Docker if unavailable.
> • *Platform differences* → Keep all platform‑specific code isolated in the `rustenium-ios` crate; the core crate remains unchanged.

### Dependencies & Prerequisites
| Tool | Version | Notes |
|------|---------|-------|
| Rust (stable) | 1.75+ | Use `cargo +nightly build -Zbuild-std` if required for macOS targets. |
| XCUITest / Appium server | v2.x | The official Rustenium‑mobile server (`rustenium-mobile`) should expose the W3C WebDriver endpoint at `http://localhost:4723/wd/hub`. |
| macOS SDK (for simulators) | Any recent | Needed for `xcrun simctl` commands. |

### What the Current Code Base Provides
- **Crate layout** – The `rustenium-ios` crate exists with a `Cargo.toml` that depends on `rustenium-core`, but its source file (`src/lib.rs`) is only a comment placeholder.
- **Examples & Tests** – `examples/ios_test.rs` and `tests/mobile.rs` call the same stub functions; they currently compile only because the symbols are declared in the crate, not implemented.

> **Conclusion:** The existing code is *not* functional. All of the above milestones will require new implementation in `crates/ios/src/lib.rs`, additional modules for capabilities, session handling, and integration with XCUITest.

### Next Steps
1. Create this markdown file at `docs/mobile/ios_implementation_plan.md`.
2. Add it to the repository and commit it.
3. Update `crates/ios/Cargo.toml` if any additional dependencies are needed (e.g., `serde`, `reqwest`).
4. Start implementing Phase 1, beginning with the capability struct.

---

*Prepared by Claude Code – 2026‑11‑07*
