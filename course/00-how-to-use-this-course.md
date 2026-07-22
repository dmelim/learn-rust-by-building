# How to Use This Course

## The learning loop

For each lesson:

1. Read the stated goal.
2. Type the small examples instead of copying them.
3. Answer the “predict” prompt before running the code.
4. Complete the project checkpoint.
5. Run `cargo fmt`, `cargo check`, and `cargo test`.
6. Write one sentence in your notes explaining what became clearer.

Compiler errors are part of the material. Read the first error in full, find
the marked source line, and consider the suggested fix before changing code.

## Commands you will use often

| Command | Purpose |
|---|---|
| `cargo new name` | Create a Cargo package |
| `cargo run` | Build and run a binary |
| `cargo check` | Check code without producing the final executable |
| `cargo test` | Build and run tests |
| `cargo fmt` | Apply standard Rust formatting |
| `cargo clippy` | Find common mistakes and unidiomatic code |

## Working with the practical

Module 1 provides two versions:

- `practical/starter`: a compiling scaffold with `TODO` markers
- `practical/solution`: one possible completed implementation

Work in the starter first. Consult the solution only after making a serious
attempt or when a hint specifically directs you to it. The solution is not the
only valid design.

## Definition of done

A practical is complete when:

- It satisfies the written behavior, including edge cases.
- `cargo fmt --check` succeeds.
- `cargo clippy -- -D warnings` succeeds.
- `cargo test` succeeds.
- You can explain each line without relying on memorized wording.

## Getting unstuck

Use this order:

1. Reduce the problem to the smallest failing line.
2. Read the complete compiler message.
3. Check the lesson’s “common traps.”
4. Search the linked Rust Book section or standard-library documentation.
5. Look at one hint.
6. Compare only the relevant function with the reference solution.
