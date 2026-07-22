# Lesson 4 — Testing Public Behavior

## Goal

Use unit tests for focused internals and integration tests for public workflows.

- Unit tests live beside code under `#[cfg(test)]` and may test private items.
- Integration tests live in `tests/` and use the library like an external user.
- Test fixtures should be isolated and deterministic.

For files, create a unique path in the system temporary directory and remove
that exact file after the assertion.

## Build the project

Add a round-trip integration test that saves and reloads orders.

Continue to [Lesson 5](05-file-io.md).
