# Lesson 2 — Iteration

## Goal

Choose whether a loop borrows, mutably borrows, or consumes collection items.

```rust
for order in &queue { /* shared borrow */ }
for order in &mut queue { /* mutable borrow */ }
for order in queue { /* consumes the vector */ }
```

Iterator adapters such as `find`, `filter`, `map`, and `count` express common
data transformations. Start with a loop when it communicates the rule better.

## Predict

Which loop forms allow the vector itself to be used afterward?

## Build the project

Count priority orders without moving them out of the queue.

Continue to [Lesson 3](03-strings.md).
