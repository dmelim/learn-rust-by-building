# Lesson 1 — Generics

## Goal

Write one operation for any type satisfying a stated contract.

```rust
fn save_records<T: Record>(records: &[T]) {
    // each T supports Record behavior
}
```

Generics are compiled into concrete code for the types used. A bound such as
`T: Record` tells both the compiler and reader which operations are available.

## Build the project

Sketch generic save and load signatures before implementing file access.

Continue to [Lesson 2](02-traits.md).
