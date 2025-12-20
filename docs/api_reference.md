# Rustenium API Reference

## Core Modules

### rustenium::webdriver

- `WebDriverHttpClient`: Async HTTP client for WebDriver protocol
- `WebDriverSession`: Manages session lifecycle, commands

### rustenium::web

- `WebElement`: Find, click, send_keys, get_text
- `BrowserCapabilities`: Chrome, Firefox

### rustenium::mobile

- `IosCapabilities`, `AndroidCapabilities`: Mobile session capabilities
- `create_ios_session`, `create_android_session`: Start mobile sessions (Appium)

### rustenium::common

- `RusteniumError`, `Result`: Error handling
- `wait_for`: Wait utility
- `capabilities`: Capability structures

### rustenium::extensions

- `bidi::BiDiClient`: BiDirectional protocol stub
- `grid::create_grid_session`: Selenium Grid remote session
- `components::define_component!`: Macro for page object modeling
- `screenshot`: Screenshot utility on `WebDriverSession`
- `logging`: Log retrieval on `WebDriverSession`

## Examples

See `/examples` and `docs/examples/` for code.
