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
* try nested trait bound: e.g. `G: Genotype<T: Gene>` and then use G and T. Maybe make type alias for it if possible
* maybe seed best_chromosome back into population after degenerate?

