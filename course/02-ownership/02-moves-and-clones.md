# Lesson 2 — Moves and Clones

## Goal

Recognize ownership transfers and duplicate heap data only when required.

```rust
let customer = String::from("Amina Nuru");
let queued_customer = customer;
// println!("{customer}"); // value was moved
```

Rust prevents two values from believing they uniquely own the same allocation.
Use `.clone()` only when two independent owned copies are genuinely needed.

## Predict

Which is cheaper: moving a `String` or cloning it? What does each operation mean
for later use of the original binding?

## Build the project: checkpoint 2

Move a completed address into a `DeliveryContact`. Then clone it deliberately
for a separate audit record and explain why that second owner is necessary.

## Common traps

- Adding `.clone()` merely to silence the compiler
- Calling a move a shallow copy
- Forgetting that passing an owned value to a function may move it

Continue to [Lesson 3](03-shared-borrowing.md).
