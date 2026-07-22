# Lesson 5 — File I/O

## Goal

Persist text records while retaining line-level error context.

```rust
let file = std::fs::File::open(path)?;
for (index, line) in BufReader::new(file).lines().enumerate() {
    let order = Order::decode(&line?)?;
}
```

File operations can fail for environmental reasons. Parsing can fail because
stored data is malformed. A storage error enum should preserve which category
occurred and, for malformed records, the line number.

The course format is intentionally simple and rejects delimiter characters in
customer names. Real systems often use an established format such as CSV or
JSON with a carefully selected dependency.

## Build the project

Save one record per line and load with contextual errors.

Continue to the [practical](06-practical-persistence.md).
