# Rustenium Documentation

Rustenium is a Rust-native, async-first end-to-end automation library for web,
iOS, and Android applications. Inspired by Selenium, it provides
a unified, high-performance, and type-safe API for browser and mobile automation.

## Documentation Sections

- **[Core](./core/README.md)**: Fundamental building blocks, sessions,
and client handling.
- **[Web Automation](./web/README.md)**: Browser automation (Chrome, Firefox, Safari).
- **[Mobile Automation](./mobile/README.md)**: iOS and Android automation via Rustenium-mobile.
- **[Testing Utilities](./testing/README.md)**: Assertions, BDD support, and reporting.
- **[CLI](./cli/README.md)**: Command-line tool for project management.

## Getting Started

1. **Installation**: Add `rustenium` to your `Cargo.toml`.
2. **Drivers**: Install required drivers (e.g., `chromedriver`, `geckodriver`, `rustenium-mobile`).
3. **Examples**: Check the [examples directory](../examples) for working code.

## Features

- **Async-first**: Built on Tokio for high-performance async execution.
- **Type-safe**: Leverages Rust's type system to prevent common runtime errors.
- **Cross-Platform**: Unified API for Web, iOS, and Android.
- **Extensible**: Support for custom capabilities and commands.

## API Reference

For detailed API documentation, see the [API Reference](./api_reference.md)
or generated Rust docs.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.
