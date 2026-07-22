# Lesson 3 — Strings in Collections

## Goal

Search and combine UTF-8 customer text safely.

```rust
fn matches_customer(order: &Order, query: &str) -> bool {
    order.customer.to_lowercase().contains(&query.to_lowercase())
}
```

This simple case-insensitive search allocates lowercase strings. It is clear
and adequate for the small queue; production Unicode search has more policy
decisions than this beginner example.

Use `join` to combine borrowed string slices for reports.

## Build the project

Return borrowed orders whose customer names contain a query.

Continue to [Lesson 4](04-hash-maps.md).
