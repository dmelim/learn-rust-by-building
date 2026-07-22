# Lesson 4 — Hash Maps

## Goal

Associate unique keys with values and aggregate repeated categories.

```rust
use std::collections::HashMap;

let counts = HashMap::new();
```

The entry API updates a value whether or not the key exists:

```rust
*counts.entry(label).or_insert(0) += 1;
```

Hash-map iteration order is not guaranteed. Sort keys before producing output
that needs deterministic ordering.

## Build the project

Count orders by status label and assert counts by key rather than map order.

Continue to [Lesson 5](05-queue-operations.md).
