# Lesson 3 — Enums

## Goal

Represent exactly one valid delivery status at a time.

```rust
enum DeliveryStatus {
    Draft,
    Ready,
    OutForDelivery { courier: String },
    Delivered,
}
```

An enum variant can carry the data that only that state requires. A courier is
therefore impossible to forget when constructing `OutForDelivery`.

## Predict

Why is an enum safer than several booleans named `ready`, `out`, and
`delivered`?

## Build the project

Add `DeliveryStatus` to `Order` and construct each variant.

Continue to [Lesson 4](04-option.md).
