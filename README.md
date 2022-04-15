# genetic-algorithm
A genetic algorithm implementation for Rust

An example usage is implemented in src/main.rs, run with `cargo run --release`

Run tests with `cargo test`

## Profiling

Currently a profiler is active in src/main.rs, run with `cargo run`

Findings:

* compete::Tournament(4) was taking 98% of the time, implemented compete::Elite for a 30x speed increase
