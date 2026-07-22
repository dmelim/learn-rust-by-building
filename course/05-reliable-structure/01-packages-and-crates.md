# Lesson 1 — Packages and Crates

## Goal

Understand how Cargo packages contain compiled crate targets.

`Cargo.toml` describes a package. `src/main.rs` is the root of a binary crate;
`src/lib.rs` is the root of a library crate. A package may contain both.

The course root is also a Cargo workspace, so one command checks every practical
without merging their code or dependencies.

## Build the project

Run `cargo metadata --no-deps` and identify package names and targets.

Continue to [Lesson 2](02-modules-and-visibility.md).
