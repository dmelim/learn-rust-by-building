# Lesson 3 — Shared Borrowing

## Goal

Let a function inspect a value without becoming its owner.

```rust
fn print_customer(name: &String) {
    println!("Customer: {name}");
}
```

`&String` is a shared reference. The function temporarily borrows the value;
the caller remains its owner and can use it afterward. Multiple shared
references may exist at the same time because none can mutate the value.

Prefer `&str` for text-only parameters; the next lessons explain why.

## Build the project: checkpoint 3

Write a function that borrows a contact and returns a formatted label. Confirm
that the caller can still use the contact after the call.

## Check your understanding

1. Who releases borrowed data?
2. Why can several readers coexist?
3. What information does `&` add to a function signature?

Continue to [Lesson 4](04-mutable-borrowing.md).
