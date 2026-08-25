# Rust Through Go and TypeScript

## Goal

Replace six familiar language translations with the Rust decisions needed for
the service-runner checkpoint.

## Compact translation guide

| Need | TypeScript | Go | Rust decision |
|---|---|---|---|
| One owner of mutable state | Conventional object ownership | Conventional pointer ownership | Put the value in the component responsible for its lifecycle |
| Temporary access | Object reference | Pointer | Borrow with `&T`, or `&mut T` for exclusive mutation |
| Missing value | `undefined` | Zero value or pointer | Use `Option<T>` and handle both variants |
| Recoverable failure | Throw or return a result shape | Return `(value, error)` | Return `Result<T, E>` and make the error part of the API |
| One of several states | Discriminated union | Constants plus fields or interfaces | Use an enum whose variants carry only valid state data |
| Shared behavior | Interface | Interface | Start concrete, then introduce a trait when multiple implementations are useful |

The syntax is the smaller difference. Rust asks you to make lifecycle and
mutation authority visible in the program structure.

## Predict the compiler

Will this compile?

```rust
let service_name = String::from("atlas-worker");
let registered_name = service_name;
println!("starting {service_name}");
```

It does not compile. Assigning the `String` moves ownership into
`registered_name`. The useful question is not whether to add `clone()` by
reflex. Decide what the second binding needs:

```rust
let service_name = String::from("atlas-worker");
let registered_name = &service_name;
println!("starting {registered_name}");
println!("configured as {service_name}");
```

Borrow when the second binding only needs temporary access. Clone when two
independent owners are genuinely required.

## Model absence explicitly

A process has no exit code while it is still running. Rust makes the distinction
visible:

```rust
fn describe_exit(code: Option<i32>) -> String {
    match code {
        Some(code) => format!("exited with code {code}"),
        None => "terminated without an exit code".to_owned(),
    }
}
```

The caller cannot accidentally treat absence as zero.

## Make failure part of the operation

Starting an already running service is a domain failure, not an exceptional
control-flow mechanism:

```rust
fn request_start(state: &mut ServiceState) -> Result<(), TransitionError> {
    state.apply(ServiceEvent::StartRequested)
}
```

The supervisor has exclusive mutation authority. `apply` changes the state only
after validating the event, and the result forces the caller to handle an
invalid request.

## Do not abstract before pressure appears

The first checkpoint launches a process with `std::process::Command` directly.
That concrete implementation will later make tests awkward. Only then will the
course introduce a trait and compare a generic boundary with `dyn Trait`.

This preserves the useful sequence:

1. Build the concrete behavior.
2. Encounter the testing or substitution problem.
3. Extract the smallest useful boundary.

## Rust judgment

For each case, choose borrowing, moving, or cloning:

1. A launcher reads a service definition during one function call.
2. A supervisor takes responsibility for a newly constructed definition.
3. A log-reading thread needs its own service identifier after the caller can
   discard the original configuration.

Suggested decisions:

1. Borrow the definition with `&ServiceDefinition`.
2. Move the definition into the supervisor.
3. Clone only the identifier that must cross into independently owned work.

Continue to [Service state and processes](02-service-state-and-processes.md).
