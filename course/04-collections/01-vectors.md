# Lesson 1 — Vectors

## Goal

Store a growable, ordered sequence of values with one element type.

```rust
let mut queue: Vec<Order> = Vec::new();
queue.push(order);
```

Indexing with `queue[0]` panics when missing. `queue.get(0)` returns an
`Option<&Order>` and makes absence explicit.

## Build the project

Create a queue, push three orders, and inspect the first with `get`.

Continue to [Lesson 2](02-iteration.md).
