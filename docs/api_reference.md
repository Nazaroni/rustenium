# Rustenium API Reference

## Core Modules

### rustenium::webdriver

- `WebDriverHttpClient`: Async HTTP client for WebDriver protocol
- `WebDriverSession`: Manages session lifecycle, commands

### rustenium::web

- `WebDriver`: High-level browser automation driver
  - `new()`, `new_chrome()`, `new_firefox()`: Create new driver instances
  - `goto(url)`, `back()`, `forward()`, `refresh()`: Navigation
  - `title()`, `url()`, `html()`: Page state
  - `find_element(selector)`, `find_elements(selector)`: Element location
  - `wait_for_element(selector, timeout)`: Wait for elements
  - `quit()`: Close browser session

- `WebElement`: Represented Web Elements
  - `click()`, `send_keys(text)`, `text()`: Interaction
  - `attribute(name)`, `property(name)`: Get attributes/properties
  - `rect()`: Get element position and size
  - `is_displayed()`, `is_enabled()`, `is_selected()`: Element state

- `WebDriverConfig`: Driver configuration
  - `base_url`, `port`, `headless`: Configuration options

- `ChromeOptions`, `FirefoxOptions`: Browser-specific options
  - `headless()`, `with_arguments(args)`: Option setting

- `ChromeCapabilities`, `FirefoxCapabilities`: Browser capabilities
  - `to_value()`: Convert to JSON value

### rustenium::mobile

- `IosCapabilities`, `AndroidCapabilities`: Mobile session capabilities
- `create_ios_session`, `create_android_session`: Start mobile sessions
  (Rustenium-mobile)

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
