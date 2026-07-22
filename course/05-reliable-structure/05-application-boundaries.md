# Lesson 5 — Application Boundaries

## Goal

Keep error reporting at the outer edge of the program.

```rust
fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
```

Domain and service functions return structured errors. `main` chooses how a
terminal user sees them. This keeps reusable logic independent of presentation.

Implement `Display` for friendly text and `Error` to participate in Rust's
standard error ecosystem.

## Build the project

Move all orchestration into `run() -> Result<(), AppError>`.

Continue to the [practical](06-practical-reliable-app.md).
