# Practical — Persistent Dispatch Data

## Requirements

1. Define a `Record` trait with encode and decode operations.
2. Implement it for `Order`.
3. Write generic `save_records` and `load_records` functions.
4. Preserve I/O failures and malformed-record line numbers.
5. Return borrowed priority-customer names with an explicit lifetime.
6. Add unit tests plus a public round-trip integration test.

Use the [starter](practical/starter/src/lib.rs), then review the
[solution](practical/solution/src/lib.rs).

## Acceptance checklist

- A saved queue loads to equal values.
- An invalid record reports its one-based line number.
- Customer names containing the delimiter are rejected.
- Temporary test files are uniquely named and cleaned up.
- The complete workspace passes all quality gates.
