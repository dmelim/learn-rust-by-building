# Practical — Typed Orders and Statuses

## Requirements

Build an order model that:

1. Owns its identifier, customer name, optional note, priority flag, and status.
2. Starts in `Draft` through `Order::new`.
3. Exposes its optional note as `Option<&str>`.
4. Produces a concise summary without consuming the order.
5. Produces a next action for every delivery status.
6. Carries a courier only in `OutForDelivery`.

Use the [starter](practical/starter/src/main.rs), then compare the
[solution](practical/solution/src/main.rs).

## Acceptance checklist

- No combination of status booleans is used.
- All variants are handled explicitly.
- Borrowed methods leave the order usable.
- Tests cover every status and optional-note state.
