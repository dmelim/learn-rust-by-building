# Lesson 4 — Error Propagation

## Goal

Compose fallible operations without deeply nested matches.

```rust
let id = parse_id(fields[0])?;
let priority = parse_priority(fields[2])?;
```

`?` returns early when the value is `Err`; otherwise it extracts the `Ok`
value. The enclosing function must return a compatible `Result` or `Option`.

## Predict

Which later operations do not run after the first error?

## Build the project

Parse an entire pipe-delimited order using small helpers and `?`.

Continue to [Lesson 5](05-application-boundaries.md).
