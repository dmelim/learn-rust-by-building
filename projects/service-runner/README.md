# Service Runner Checkpoints

This project grows through explicit, runnable checkpoints. Each directory is a
complete stage that preserves the design decisions introduced by its lesson.

## Available checkpoint

- [`01-state-and-processes`](01-state-and-processes): model a service lifecycle,
  launch one named child process, and record its process identifier and exit code

Run the current checkpoint from the repository root:

```console
cargo test -p service_runner_state
cargo run -p service_runner_state
```

The next checkpoint will add concurrent stdout and stderr capture. It should be
added only after the ownership and shutdown responsibilities are written down.
