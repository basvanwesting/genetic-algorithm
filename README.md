# genetic-algorithm
A genetic algorithm implementation for Rust

Run examples with e.g. `cargo run --example evolve_binary --release`

See example of custom fitness function in the example evolve_nqueens.

## Tests

Run tests with `cargo test`

## Benchmarks

Run benchmarks with `cargo bench`

## Profiling

`cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

find the flamegraph in: ./target/criterion/profile_evolve_binary/profile/flamegraph.svg

## TODO
* generalize over Range and RangeInclusive for RangeGenotype if possible. rand has a SampleRange<T> Trait, which is only implemented for Range and RangeInclusive, so that might be nice
* remove builder duplication in Genotype implementations
* maybe seed best_chromosome back into population after degenerate?

