# Lesson 2 — Traits

## Goal

Define behavior that multiple record types could implement.

```rust
trait Record: Sized {
    fn encode(&self) -> String;
    fn decode(line: &str) -> Result<Self, String>;
}
```

A trait is a behavior contract, not inheritance of stored fields. `Self`
refers to the implementing type; `Sized` allows `decode` to return it directly.

## Build the project

Implement `Record` for `Order` with a documented pipe-delimited format.

Continue to [Lesson 3](03-lifetimes.md).
