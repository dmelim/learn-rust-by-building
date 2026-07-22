# Module 5 — Reliable Program Structure

## Outcome

Split the queue into focused modules and report recoverable failures with
`Result` instead of panicking.

This module follows [Rust Book Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
and [Chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html).

## Lesson sequence

| Lesson | Main idea | Checkpoint |
|---|---|---|
| [1. Packages and crates](01-packages-and-crates.md) | Cargo boundaries | Identify targets |
| [2. Modules and visibility](02-modules-and-visibility.md) | Organize responsibilities | Split domain and service |
| [3. Recoverable errors](03-recoverable-errors.md) | `Result<T, E>` | Parse order data |
| [4. Error propagation](04-error-propagation.md) | `?` and conversions | Compose fallible work |
| [5. Application boundaries](05-application-boundaries.md) | Report errors once | Keep `main` small |
| [Practical](06-practical-reliable-app.md) | Combined structure | Reliable queue import |
