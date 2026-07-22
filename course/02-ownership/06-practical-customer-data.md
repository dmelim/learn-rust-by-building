# Practical — Customer and Address Data

## Scenario

The dispatch desk needs a printable label. Customer and address data must move
into one record, be borrowed for formatting, and be corrected without cloning
the entire record.

## Requirements

1. Define `DeliveryContact` with owned `String` fields.
2. Normalize repeated whitespace in names and addresses.
3. Return the first word as a borrowed `&str`.
4. Format a label by borrowing the contact.
5. Correct the customer name through `&mut DeliveryContact`.
6. Keep the contact usable after every borrowed call.

Start with [the scaffold](practical/starter/src/main.rs) and compare with the
[reference solution](practical/solution/src/main.rs) after completing it.

## Acceptance checklist

- All tests pass without unnecessary `.clone()` calls.
- New normalized text is owned; views into existing text are borrowed.
- Formatting does not consume the record.
- The code passes formatting and Clippy checks.

## Reflection

For every function, say who owns each argument before and after the call.
