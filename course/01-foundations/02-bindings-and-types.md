# Lesson 2 — Bindings and Types

## Goal

Represent delivery facts with appropriate types and turn those facts into a
calculation.

## Values have types

Rust is statically typed: the compiler determines each value’s type before the
program runs. It can often infer the type from how a value is used.

```rust
let distance_km: f64 = 12.0;
let average_speed_kmh: f64 = 30.0;
let preparation_minutes: u32 = 10;
let is_priority: bool = false;
```

Useful scalar types for this project:

| Type | Represents | Example |
|---|---|---|
| `u32` | Non-negative whole number | `10` minutes |
| `i32` | Whole number that may be negative | `-2` degrees |
| `f64` | Floating-point number | `12.5` kilometres |
| `bool` | `true` or `false` | priority service |
| `char` | One Unicode scalar value | `'A'` |

A numeric type is part of the program’s meaning. A distance can contain a
fraction, while a displayed number of minutes can be a whole number.

## Immutable by default

```rust
let distance_km = 12.0;
distance_km = 15.0; // does not compile
```

A binding cannot be reassigned unless it uses `mut`:

```rust
let mut completed_deliveries = 0;
completed_deliveries += 1;
```

Prefer immutable bindings until the algorithm genuinely requires reassignment.
This narrows the number of places where a value can change.

## Constants and shadowing

A constant has an explicit type and can be declared outside a function:

```rust
const MINUTES_PER_HOUR: f64 = 60.0;
```

Shadowing creates a new binding with the same name:

```rust
let input = " 12.5 ";
let input = input.trim();
```

The second `input` is a new value, not a mutation of the first one.

## Calculate travel time

```rust
const MINUTES_PER_HOUR: f64 = 60.0;

fn main() {
    let distance_km = 12.0;
    let average_speed_kmh = 30.0;

    let travel_minutes =
        (distance_km / average_speed_kmh * MINUTES_PER_HOUR).ceil() as u32;

    println!("Travel time: {travel_minutes} minutes");
}
```

The calculation first produces an `f64`. Calling `.ceil()` rounds upward because
a delivery estimate should not promise less than a partial minute. `as u32`
then converts the rounded value for display and later whole-minute arithmetic.

## Predict

What value and type does each binding have?

```rust
let distance = 15.0;
let speed = 40.0;
let exact_minutes = distance / speed * 60.0;
let shown_minutes = exact_minutes.ceil() as u32;
```

Run the code with a `println!` after making your prediction.

## Build the project: checkpoint 2

Replace the fixed estimate from checkpoint 1 with a calculation:

1. Bind distance, average speed, and preparation minutes.
2. Calculate travel minutes.
3. Add preparation and travel minutes.
4. Print all inputs and the resulting estimate.

Try these cases:

| Distance | Speed | Preparation | Expected total |
|---:|---:|---:|---:|
| 12 km | 30 km/h | 10 min | 34 min |
| 5 km | 20 km/h | 8 min | 23 min |

## Common traps

- Dividing integers when a fractional result is required
- Mixing `f64` and `u32` directly in one arithmetic expression
- Using `mut` for every binding
- Converting to an integer before completing the floating-point calculation

## Check your understanding

1. Why are bindings immutable by default?
2. Why is distance an `f64` but the displayed estimate a `u32`?
3. What information could be lost during `as u32` conversion?

Continue to [Lesson 3](03-functions-and-expressions.md).
