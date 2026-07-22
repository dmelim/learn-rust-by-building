# Lesson 5 — Pattern Matching

## Goal

Handle every status and destructure associated data.

```rust
match &order.status {
    DeliveryStatus::Draft => "complete the order".to_owned(),
    DeliveryStatus::Ready => "assign a courier".to_owned(),
    DeliveryStatus::OutForDelivery { courier } => format!("track {courier}"),
    DeliveryStatus::Delivered => "archive".to_owned(),
}
```

The compiler reports a missing variant when the match is not exhaustive. Use
`if let` when only one pattern matters and all others share the same behavior.

## Build the project

Return a next-action message for every status and test each branch.

Continue to the [practical](06-practical-orders.md).
