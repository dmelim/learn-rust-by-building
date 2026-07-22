> **AI-assisted course experiment:** This course was created with substantial
> assistance from OpenAI GPT-5.6 Sol to explore whether an AI system can
> structure a coherent, engaging programming curriculum. The code, exercises,
> and links are tested, but the material should still be approached critically.

# Learn Rust by Building

A beginner-first Rust course built around one growing project: a small delivery
dispatch desk.

> Learn one idea, use it immediately, and carry it into a larger program.

The official [Rust Book](https://doc.rust-lang.org/book/) is the conceptual
backbone. This course reorganizes its early material into shorter learning
loops with predictions, experiments, practical exercises, compiling starter
projects, and tested reference solutions.

## Open the course

The repository includes a responsive HTML course reader that renders the
Markdown lessons and Mermaid diagrams locally. No npm install or CDN connection
is required.

From Git Bash, macOS, or Linux:

```console
bash scripts/serve-course.sh
```

Or, anywhere Node.js is available:

```console
node site/static-server.mjs 8000
```

Then open <http://localhost:8000>.

You can also read the [curriculum directly in Markdown](course/README.md) and
start with [How to use this course](course/00-how-to-use-this-course.md).

## What you will build

The course starts with a delivery-time calculation and grows it into a tested,
persistent command-line application.

| Module | Main Rust ideas | Practical milestone |
|---|---|---|
| [1. Foundations](course/01-foundations/README.md) | Cargo, bindings, functions, control flow, input | Interactive delivery estimate |
| [2. Ownership](course/02-ownership/README.md) | Ownership, moves, borrowing, slices, `String` | Safe customer and address data |
| [3. Domain modeling](course/03-domain-modeling/README.md) | Structs, methods, enums, `Option`, matching | Typed orders and statuses |
| [4. Collections](course/04-collections/README.md) | Vectors, iteration, strings, hash maps | Searchable delivery queue |
| [5. Reliable structure](course/05-reliable-structure/README.md) | Modules, visibility, `Result`, propagation | Modular, error-aware application |
| [6. Tests, traits, and files](course/06-tests-traits-files/README.md) | Generics, traits, lifetimes, tests, file I/O | Persistent dispatch data |
| [Capstone](course/capstone/README.md) | Integration and refactoring | Dispatch Desk CLI |

Every module is complete and includes lessons, a practical brief, a starter,
and a reference solution.

## How the practicals work

1. Read the lesson and predict what each example will do.
2. Open that module's `practical/starter` project in your editor.
3. Complete the TODOs and enable the relevant ignored tests as instructed.
4. Run the package tests and use compiler feedback to iterate.
5. Compare behavior with `practical/solution` after making your own attempt.

The capstone follows the same pattern in `course/capstone/starter` and
`course/capstone/solution`.

## Requirements

- Rust 1.90 or newer
- Cargo
- A terminal and a code editor
- Node.js or Python only if you want to serve the HTML reader locally

All Rust projects use Rust 2024 Edition and only Rust's standard library.

Check your Rust installation with:

```console
rustc --version
cargo --version
```

## Repository structure

```text
course/                   lessons, practicals, and capstone
  01-foundations/
  ...
  06-tests-traits-files/
  capstone/
scripts/                  local validation and course-server commands
site/                     course reader, styles, and vendored Mermaid runtime
index.html                browser entry point
Cargo.toml                workspace containing all practical projects
```

## Validate the repository

Run the same formatting, linting, test, link, and course-reader checks used by
continuous integration:

```console
bash scripts/check.sh
```

The individual Rust commands are:

```console
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Example data and privacy

All customer names, addresses, order records, and identifiers in the lessons,
starter projects, solutions, and tests are fictional fixtures created for this
course. They are intentionally drawn from varied regions and must not be
replaced with real customer or personal data.

## Contributing

Contributions that make explanations clearer, improve exercises, or catch
incorrect behavior are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) before
opening a change.

## License

Software and code examples are dual-licensed under
[MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option. Original
course prose, exercises, and diagrams are licensed under
[CC BY 4.0](LICENSE-CC-BY-4.0). See [LICENSE.md](LICENSE.md) for the complete
scope and third-party notices.
