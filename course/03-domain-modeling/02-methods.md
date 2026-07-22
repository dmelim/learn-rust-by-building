# Lesson 2 — Methods

## Goal

Place behavior beside the type whose meaning it uses.

```rust
impl Order {
    fn summary(&self) -> String {
        format!("#{} — {}", self.id, self.customer)
    }
}
```

`&self` is shared borrowing written in method form. Associated functions omit
`self` and are commonly used as constructors.

## Build the project

Add `Order::new` and `summary`. Decide which parameters must be owned.

## Common traps

- Taking `self` when the method only needs to inspect
- Writing getters for every field without a use case
- Hiding unrelated utility behavior in an `impl`

Continue to [Lesson 3](03-enums.md).
