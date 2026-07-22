# Lesson 4 — Optional Data

## Goal

Represent missing data explicitly with `Option<T>`.

```rust
struct Order {
    delivery_note: Option<String>,
    // other fields
}
```

`Some(value)` means data is present; `None` means it is absent. Rust has no
general null value, so code must handle absence before using the inner value.

Use `as_deref()` to turn `Option<String>` borrowed through `&self` into
`Option<&str>`.

## Build the project

Add an optional delivery note and a method returning `Option<&str>`.

Continue to [Lesson 5](05-pattern-matching.md).
