# Practical — Reliable Queue Import

## Input format

Each order is `id|customer|priority`, where priority is `yes` or `no`.

## Requirements

1. Split domain, error, and service responsibilities into modules.
2. Parse valid rows into `Order`.
3. Return a specific `AppError` for every invalid field.
4. Dispatch by identifier or return `OrderNotFound`.
5. Use `?` to propagate failures to `run`.
6. Print errors only at the application boundary.

Use the [starter](practical/starter/src/main.rs), then review the
[solution](practical/solution/src/main.rs).

## Acceptance checklist

- Expected bad data never panics.
- Error messages include useful offending values.
- Modules expose only what their callers need.
- Unit tests cover each failure category.
