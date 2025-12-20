# Testing Utilities

Rustenium includes crates to simplify writing and running tests.

## Crates

- **assertions**: Provides a set of fluent assertions for validating test
  results.
- **bdd**: Supports Behavior-Driven Development (BDD) style tests with Gherkin
  syntax.
- **reporter**: Generates test reports in various formats (e.g., JSON, HTML).

## Example (Assertions)

```rust
use rustenium_assertions::assert_that;

assert_that!(page_title).is_equal_to("Example Domain");
```
