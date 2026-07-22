# Lesson 3 — Lifetimes at the Boundary

## Goal

Describe which input borrow keeps returned references valid.

```rust
fn priority_customers<'a>(orders: &'a [Order]) -> Vec<&'a str> {
    orders.iter().filter(|order| order.priority)
        .map(|order| order.customer.as_str()).collect()
}
```

The annotation does not extend any lifetime. It states that returned slices are
borrowed from `orders` and cannot outlive it. Rust often infers this relationship
for one input reference; the explicit version makes the contract visible.

## Build the project

Return customer-name views without allocating or cloning names.

Continue to [Lesson 4](04-testing.md).
