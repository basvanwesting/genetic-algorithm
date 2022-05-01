# genetic-algorithm
A genetic algorithm implementation for Rust

## Genotypes
One cool thing to do with genotypes is to make a meta-genotype of all the Crossover/Mutate/Compete and other Evolve parameters. This could be used to optimize the parameter of some other genetic algorithm.
Yes, a simple nested for loop would also work, but where is the fun in that?
But I wasn't able to find an elegant approach to creating a heterogene setup. It was tried with Trait objects, Any and Enums, but all didn't work well:

* Genotype wasn't allowed to become a Trait object due to it's other traits and generics.
* Any worked, but you still need to know all possible Genotypes up front for downcasting, defeating the flexible purpose
* Enum worked, but you still need to know all possible Genotypes up front for wrapping, defeating the flexible purpose

So, after some consideration I settled on using an index based Genotype with an external vector of arbitrary types which should be retrieved in the fitness function.

Implemented genotypes:

* BinaryGenotype: list of true|false values with 50% chance, permutable. Initialize with only gene size.
* ContinuousGenotype: list of 0.0..1.0 values with uniform chance, with optional offset and scale, not-permutable. Initialize with only gene size.
* IndexGenotype: list of 0..n with uniform chance, permutable. Initialize with gene size and n.
* UniqueIndexGenotype: list of 0..n with uniform chance, each index occurs exactly once, permutable. Initialize with gene size and n.
* MultiIndexGenotype: list of 0..x, 0..y, 0..z, etc... where each gene has it's own index range with a weighted chance depending on the range size. Initialize list of max-index values.

Discarded genotypes:

* DiscreteGenotype with arbitrary list of normal numbers, permutable
* DiscreteGenotype with arbitrary list of real numbers, permutable
* UniqueDiscreteGenotype with arbitrary list of normal numbers, each index number exactly once, ppermutable
* UniqueDiscreteGenotype with arbitrary list of real numbers, each index number exactly once, ppermutable
* RangeGenotype with arbitrary normal number range, permutable
* RangeGenotype with arbitrary real number range, not-permutable
* UniqueRangeGenotype with arbitrary normal number range, each number occurs exactly once, not permutable

## Examples
Run examples with e.g. `cargo run --example evolve_binary --release`

See example of custom fitness function in the example evolve_nqueens.

## Tests
Run tests with `cargo test`

## Benchmarks
Implemented using criterion.
Run benchmarks with `cargo bench`

## Profiling
Implemented using criterion and pprof. find the flamegraph in: ./target/criterion/profile_evolve_binary/profile/flamegraph.svg

`cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

## TODO
* run clippy
* remove builder duplication in Genotype implementations
* maybe seed best_chromosome back into population after degenerate?
* make meta-genotype example to optimize nqueens configuration
