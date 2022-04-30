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
* Make Gene trait attribute of Genotype. This was we can eventually create a heterogene Chromosome (current chromosome is homogene). Maybe drop Gene trait, and directly implement Gene as associated type on Genotype as u32 etc. This due to naming issues (Gene, GeneT GeneType, Nucleotide, Allele) these all don't work nicely
* run clippy
* generalize over Range and RangeInclusive for RangeGenotype if possible. rand has a SampleRange<T> Trait, which is only implemented for Range and RangeInclusive, so that might be nice
* remove builder duplication in Genotype implementations
* maybe seed best_chromosome back into population after degenerate?
* RangeGenotype with steps? (makes it permutable for floats as well)
* use dyn Trait for Evolve and Permutate, as there is almost no performance overhead for single dynamic lookup and it cleans up the Evolve generics quite a lot
