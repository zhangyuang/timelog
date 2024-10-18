# timelog

A simple timer library for logging time durations, similar to console.time in JavaScript.

## Features

- Create multiple named timers
- Start, log, and end timers
- Print timing results in milliseconds
- Support for optional additional messages

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
timelog = "1.0.0"
```

## Usage

```rust
use timelog::Timer;

fn main() {
    let mut timer = Timer::new();

    timer.time("fetch_data");

    let duration: f64 = timer.time_log("fetch_data", true);
    println!("fetch_data took {:.2}ms", duration);
}
```

## Documentation

[timelog](https://docs.rs/timelog)
