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

See example meta_evolve_binary for an meta analysis of the evolution strategy. `cargo run --example meta_evolve_binary --release`
Currently implemented as a permutation, but with caching an evolve strategy could also be used for larger search spaces.

Implemented genotypes:

* BinaryGenotype: list of true|false values with 50% chance, permutable. Initialize with only gene-size.
* ContinuousGenotype: list of 0.0..1.0 values with uniform chance, with optional offset and scale, not-permutable. Initialize with only gene-size.
* IndexGenotype: list of 0..n with uniform chance, permutable. Initialize with gene-size and gene-value-size (= n).
* UniqueIndexGenotype: list of 0..n with uniform chance, each index occurs exactly once, permutable. Initialize with only gene-size (= gene-value-size = n).
* MultiIndexGenotype: list of 0..x, 0..y, 0..z, etc... where each gene has it's own index range with a weighted chance depending on the range size. Initialize with list of gene-value-sizes (= [x,y,x,etc...]).

Discarded genotypes:

* DiscreteGenotype with arbitrary list of normal numbers, permutable
* DiscreteGenotype with arbitrary list of real numbers, permutable
* UniqueDiscreteGenotype with arbitrary list of normal numbers, each index number exactly once, ppermutable
* UniqueDiscreteGenotype with arbitrary list of real numbers, each index number exactly once, ppermutable
* RangeGenotype with arbitrary normal number range, permutable
* RangeGenotype with arbitrary real number range, not-permutable
* UniqueRangeGenotype with arbitrary normal number range, each number occurs exactly once, not permutable

## Examples

See example evolve_nqueens for a custom fitness function. `cargo run --example evolve_binary --release`

## Tests
Run tests with `cargo test`

## Benchmarks
Implemented using criterion.
Run benchmarks with `cargo bench`

## Profiling
Implemented using criterion and pprof. find the flamegraph in: ./target/criterion/profile_evolve_binary/profile/flamegraph.svg

`cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

## TODO
* Run clippy
* Remove builder duplication in Genotype implementations
* Maybe seed best_chromosome back into population after degenerate?
* Add optional offset and scale to ContinuousGenotype
* Make duration stats return Duration, so we can choose sec/milli/micro afterwards.
* Make meta/fitness and meta/permutate use a config reference, need lifetimes
* Add factor parameter between best_fitness_score_mean and duration_mean_subsec_micros in meta/fitness
* Add DiscreteGenotype back in for user defined Gene
* Make fitness/simple_sum generic

## REJECTED TODO
* Make permutation_factory return an iterator instead of a vec => no interface by population

