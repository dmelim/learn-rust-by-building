# Lesson 1 — Structs

## Goal

Give related order data one meaningful type.

```rust
struct Order {
    id: u32,
    customer: String,
    priority: bool,
}
```

A struct makes invalid field combinations easier to notice and lets functions
accept one domain value instead of parallel arguments.

## Predict

After moving an owned `String` into `Order`, which value owns the allocation?

## Build the project

Construct an order using field-init shorthand and print selected fields.

Continue to [Lesson 2](02-methods.md).
