# Lesson 5 — String Slices

## Goal

Borrow part or all of UTF-8 text without allocating another `String`.

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
```

`&str` can refer to a string literal, an entire `String`, or part of one. It is
usually the most flexible borrowed-text parameter.

Avoid indexing strings by numeric position: UTF-8 characters occupy varying
numbers of bytes. Use methods such as `split_whitespace`, `chars`, or verified
byte boundaries.

## Build the project: checkpoint 5

Accept `&str` in a whitespace-normalization function and return an owned
`String`, because the normalized result is new text.

## Check your understanding

1. Why can `&str` accept more callers than `&String`?
2. When must a function return owned text?
3. Why is arbitrary string indexing unsafe?

Continue to the [practical](06-practical-customer-data.md).
