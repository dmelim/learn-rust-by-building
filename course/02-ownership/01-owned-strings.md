# Lesson 1 — Owned Strings

## Goal

Distinguish copyable fixed-size values from owned, growable text.

```rust
let minutes: u32 = 34;
let customer = String::from("Amina Nuru");
```

The integer has a known fixed size. A `String` is a small value that owns a
growable UTF-8 buffer. When its owner leaves scope, Rust releases that buffer.

## Predict

Why can `minutes` be assigned to another binding and still be used, while doing
the same with `customer` transfers ownership?

## Build the project: checkpoint 1

Create owned customer-name and address values. Print both, then watch where each
binding's scope ends.

## Common traps

- Confusing string literals (`&str`) with owned `String` values
- Assuming garbage collection is required to release a `String`
- Treating all assignments as deep copies

Continue to [Lesson 2](02-moves-and-clones.md).
