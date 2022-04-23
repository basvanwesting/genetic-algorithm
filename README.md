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
* seed best_chromosome back into population after degenerate?
* maybe make crossover and compete optional as nqueens doesn't really need it
* add DiscreteRangeUnique, and DiscreteRangeRandom for easier initialization of genotype for large ranges

