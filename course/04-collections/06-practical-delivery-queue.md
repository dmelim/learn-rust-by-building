# Practical — Delivery Queue

## Requirements

1. Store orders in a `Vec<Order>`.
2. Search customer names case-insensitively without consuming the queue.
3. Count orders by status using `HashMap`.
4. Remove the earliest priority order, falling back to FIFO order.
5. Return `None` when no work remains.
6. Print report rows in deterministic order where appropriate.

Use the [starter](practical/starter/src/main.rs), then review the
[solution](practical/solution/src/main.rs).

## Acceptance checklist

- Empty, standard-only, and mixed-priority queues are tested.
- Searches borrow their results.
- Tests never rely on hash-map iteration order.
- Formatting, Clippy, and tests pass.
