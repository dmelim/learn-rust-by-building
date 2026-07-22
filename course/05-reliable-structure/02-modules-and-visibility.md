# Lesson 2 — Modules and Visibility

## Goal

Group code by responsibility and expose only intentional interfaces.

```rust
mod domain;
mod error;
mod service;
```

Items are private by default. `pub` exposes an item to its parent module;
`pub(crate)` exposes it within the current crate. Keep fields private when
constructors and methods can preserve stronger rules.

## Build the project

Move `Order`, `AppError`, and parsing/dispatch behavior into separate files.

Continue to [Lesson 3](03-recoverable-errors.md).
