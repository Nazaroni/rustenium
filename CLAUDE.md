# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Quick Commands

| Action | Command | Notes |
|--------|---------|-------|
| Build all crates | `cargo build` | Compiles the entire workspace; use `-p <crate>` to target a single crate. |
| Format code | `cargo fmt` | Uses rustfmt on every crate. |
| Lint code | `cargo clippy --all-targets --all-features` | Runs clippy across the workspace. |
| Run all tests | `cargo test` | Executes unit and integration tests for each crate. |
| Run a single test file | `cargo test --test <name>` (e.g., `cargo test --test ios_test`) | Test binaries are located in `tests/` or `examples/`. |
| Run CLI tool | `cargo run -p rustenium-cli -- <args>` | Launches the command‑line interface. |
| Generate docs | `cargo doc --open` | Builds HTML docs for all crates and opens them. |

## Workspace Layout & High‑Level Architecture

The repository is a Cargo workspace with the following key crates:

- **`rustenium-core`** – Shared traits, session management, WebDriver protocol abstractions.
- **`rustenium-web`** – Browser automation implementation (Chrome, Firefox, Safari). Exposes `WebDriver` types used by higher‑level crates.
- **`rustenium-mobile`** – Unified mobile API. Internally routes to Android or iOS implementations.
- **`rustenium-ios` / `rustenium-android`** – Platform‑specific adapters that talk to XCUITest/UiAutomator2 servers.
- **`rustenium-cli`** – CLI wrapper around the core functionality, providing a `rustenium` binary.
- **`rustenium-reporter`** – Test reporting utilities.
- **`rustenium-bdd` & `rustenium-assertions`** – BDD runner and assertion library for writing expressive tests.

The *core* crate defines the WebDriver session API, JSON serialization of capabilities, and common error handling. The *web* and *mobile* crates each provide concrete implementations that spawn drivers (e.g., `chromedriver`, `rustenium‑mobile` server) and translate high‑level commands into HTTP requests.

Examples in `examples/` demonstrate how to use the public API:
- `ios_test.rs` – creates an iOS session via XCUITest.
- `android_test.rs` – creates an Android session via UiAutomator2.
- `web_example.rs` – drives a browser with `WebDriver::new_chrome()`.

The tests in the `tests/` directory exercise both unit logic and integration against real drivers. For mobile tests, ensure that the appropriate driver (e.g., XCUITest server or Android emulator) is running on the expected port (`4723` by default).

## Development Tips
- When adding new functionality, run `cargo test --all-features` to ensure no existing behaviour is broken.
- Use `cargo watch -x 'test'` for continuous testing during development.
- For mobile‑specific changes, modify the corresponding crate under `crates/ios` or `crates/android`; recompile with `cargo build -p rustenium-ios` etc.

## Herdr Integration
Herdr is a desktop terminal and coding agent interface that renders its UI from JSON layout files (`layouts/*.json`) and JavaScript mod modules in `mods/`. The repository includes an example mod (`mods/agent_comm/mod.js`) that demonstrates how to use Herdr’s runtime APIs to interact with the built‑in `hc` command‑line tool.

### Running Herdr
If you have a Herdr build installed, start it from the workspace root:
```bash
herdr-app
```
Herdr will load its default layouts and mods. To add the example mod:
1. Copy the `mods/agent_comm` directory into your Herdr data folder (usually `~/.config/herdr-app/mods`).
2. Add a reference to it in a layout, e.g., in `layouts/main.json`:
   ```json
   {
     "type": "mod",
     "mod": "agent_comm",
     "props": {}
   }
   ```
3. Reload Herdr or press the keybinding you set (see below).

### Keybindings for the Mod
You can bind a hotkey to open the agent‑communication panel by adding an entry in `keys.json`:
```json
{
  "Ctrl-Alt-A": {
    "command": "herdr-app do mod.run agent_comm"
  }
}
```
Pressing **Ctrl‑Alt‑A** will launch the panel, showing a list of running agents and buttons to send messages.

### Using Herdr’s Runtime APIs
The example mod uses two main helpers:
- `herdr.runCommand(cmd)` – runs an arbitrary shell command in Herdr’s environment and returns stdout.
- `herdr.getInfo()` – provides metadata about the current Herdr session (e.g., window name).
These are documented in the local type definition files (`gpui-kit.d.ts` and `herdr.d.ts`).

### Debugging
Herdr includes a built‑in dev console (Ctrl‑Shift‑I). You can inspect JavaScript errors or log output from your mods there.

---

**End of CLAUDE.md**