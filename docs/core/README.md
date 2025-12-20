# Core Components

The `core` module provides the fundamental building blocks for Rustenium.

## Modules

- **Client**: Handles HTTP communication with WebDriver servers.
- **Session**: Manages the lifecycle of a WebDriver session.
- **Capabilities**: Defines the capabilities required for a session (e.g.,
  browser name, platform).
- **Common**: Shared utilities and types used across the library.

## Usage

Most users will interact with `core` indirectly through the `web` or `mobile`
crates, but it can be used directly for low-level WebDriver interactions.
