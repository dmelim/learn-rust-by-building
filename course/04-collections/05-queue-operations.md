# Lesson 5 — Queue Operations

## Goal

Translate dispatch policy into collection operations.

The desk serves the earliest priority order first, otherwise the first order.
Find its position, then remove it:

```rust
let index = queue.iter().position(|order| order.priority).unwrap_or(0);
let next = queue.remove(index);
```

`Vec::remove` shifts later elements, which is acceptable for this small course
project. Different scale or policy might call for `VecDeque` or a priority heap.

## Build the project

Return `None` for an empty queue and `Some(Order)` otherwise.

Continue to the [practical](06-practical-delivery-queue.md).
