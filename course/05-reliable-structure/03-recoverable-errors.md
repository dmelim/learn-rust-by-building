# Lesson 3 — Recoverable Errors

## Goal

Return expected failure as data.

```rust
fn parse_id(text: &str) -> Result<u32, AppError> {
    text.parse().map_err(|_| AppError::InvalidId(text.to_owned()))
}
```

Use `panic!` for violated internal assumptions, not malformed user or file data.
A custom enum makes callers handle meaningful categories instead of matching
fragile message strings.

## Build the project

Define errors for field count, identifier, priority, and missing order.

Continue to [Lesson 4](04-error-propagation.md).
