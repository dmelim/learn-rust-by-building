# Lesson 4 — Mutable Borrowing

## Goal

Change owned data temporarily through one exclusive reference.

```rust
fn rename(customer: &mut String, new_name: &str) {
    customer.clear();
    customer.push_str(new_name);
}
```

While a mutable reference is active, no other reference may access the same
value. This prevents data races and invalidated views at compile time.

## Predict

Why does Rust reject reading `customer` while a later use of `&mut customer`
keeps the mutable borrow active?

## Build the project: checkpoint 4

Add a correction function that mutably borrows a contact. Keep the borrow's
scope as short as possible.

## Common traps

- Forgetting both the binding and reference need appropriate mutability
- Creating a second reference before the mutable borrow's last use
- Returning references to temporary local data

Continue to [Lesson 5](05-string-slices.md).
