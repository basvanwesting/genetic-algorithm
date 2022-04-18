# genetic-algorithm
A genetic algorithm implementation for Rust

Run examples with e.g. `cargo run --example evolve_binary --release`

## Tests

Run tests with `cargo test`

## Benchmarks

Run benchmarks with `cargo bench`

## Profiling

`cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

find the flamegraph in: ./target/criterion/profile_evolve_binary/profile/flamegraph.svg

Findings:

* compete::Tournament(4) was taking 98% of the time, implemented compete::Elite for a 30x speed increase
* in non-release mode fitness::SimpleSum is the new bottleneck, but in release mode it is not
