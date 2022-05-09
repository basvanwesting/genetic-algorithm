# genetic-algorithm
A genetic algorithm implementation for Rust

## Usage

### Evolve

#### Usage
```rust
// need randomness, simply inject for maximum flexibility and testability
let mut rng = SmallRng::from_entropy();

let genotype = BinaryGenotype::builder() // boolean genes
    .with_gene_size(100)                 // 100 of them
    .build()
    .unwrap();

println!("{}", genotype);

let evolve = Evolve::builder()
    .with_genotype(genotype)
    .with_population_size(100)          // evolve with 100 chromosomes
    .with_target_fitness_score(100)     // goal is 100 times true in the best chromosome
    .with_crossover(CrossoverAll(true)) // crossover all individual genes between 2 chromosomes for offspring
    .with_mutate(MutateOnce(0.2))       // mutate a single gene with a 20% chance
    .with_fitness(FitnessSimpleCount)   // count the number of true values in the chromosomes
    .with_compete(CompeteElite)         // sort the chromosomes by fitness to determine crossover order
    .build()
    .unwrap()
    .call(&mut rng);

println!("{}", evolve);
```

### Permutate
Sometimes the population size is small enough to simply check all possible solutions.
No randomness, mutation, crossover, competition strategies needed.

#### Usage
```rust
let genotype = BinaryGenotype::builder()
    .with_gene_size(16)
    .build()
    .unwrap();

println!("{}", genotype);

let permutate = Permutate::builder()
    .with_genotype(genotype)
    .with_fitness(FitnessSimpleCount)                 // count true values for fitness
    .with_fitness_ordering(FitnessOrdering::Minimize) // goal is zero true values
    .build()
    .unwrap()
    .call();

println!("{}", permutate);
```


## Genotypes
Implemented genotypes:

* `BinaryGenotype`: list of true|false values with 50% chance, permutable. Initialize with only gene-size.
* `DiscreteGenotype<T>`: list of n items with uniform chance, permutable. Initialize with gene-size and gene-values (of size n).
* `UniqueDiscreteGenotype<T>`: list of n items with uniform chance, each item occurs exactly once, permutable. Initialize with only gene-values (of size n = gene-size).
* `MultiDiscreteGenotype<T>`: list of separate item lists (different sizes, but same type), where each gene has it's own item list with a weighted chance depending on the list size. Initialize with list of gene-values (= [[x1, x2],[y1, y2, y3], ...]).
* `ContinuousGenotype`: range of n float values (f32) with uniform chance, not-permutable. Initialize with gene-size(= n) and gene-range.
* `MultiContinuousGenotype`: list of separate float ranges (different sizes, but same type = f32), where each gene has it's own range with a weighted chance depending on the range size. Initialize with list of gene-ranges (= [(0..1), (0..3), (2..5), ...]).

## Examples

* N-Queens puzzle https://en.wikipedia.org/wiki/Eight_queens_puzzle.
    * `UniqueDiscreteGenotype<u8>` with a 64x64 chess board setup and custom `NQueensFitness` fitness
    * `cargo run --example evolve_nqueens --release`
* Knapsack problem: https://en.wikipedia.org/wiki/Knapsack_problem 
    * `DiscreteGenotype<(weight, value)>` with a custom `KnapsackFitness(weight_limit)` fitness
    * `cargo run --example evolve_knapsack --release`
* Infinite Monkey theorem: https://en.wikipedia.org/wiki/Infinite_monkey_theorem 
    * `DiscreteGenotype<u8>` 100 monkeys randomly typing characters in a loop
    * `cargo run --example evolve_monkeys --release`
* Custom Fitness function with LRU cache
    * Note: doesn't help performance much in this case...
    * `cargo run --example evolve_binary_lru_cache_fitness --release`

## Heterogeneous Genotypes & Meta performance analysis

One cool thing to do with genotypes is to make a meta-genotype of all the Crossover/Mutate/Compete and other Evolve parameters. This could be used to optimize the parameter of some other genetic algorithm.
Yes, a simple nested for loop would also work, but where is the fun in that?
But I wasn't able to find an elegant approach to creating a heterogene setup. It was tried with Trait objects, Any and Enums, but all didn't work well:

* Genotype wasn't allowed to become a Trait object due to it's other traits and generics.
* Any worked, but you still need to know all possible Genotypes up front for downcasting, defeating the flexible purpose
* Enum worked, but you still need to know all possible Genotypes up front for wrapping, defeating the flexible purpose

So, after some consideration I settled on using an nested index based Genotype with an external vector of arbitrary types which should be retrieved in the fitness function: `MultiDiscreteGenotype<usize>`.

See example meta_evolve_binary for an meta analysis of the evolution strategy:
    * `cargo run --example meta_evolve_binary --release`
    * `cargo run --example meta_evolve_nqueens --release`

Currently implemented as a permutation, but with caching an evolve strategy could also be used for larger search spaces.

```rust
let rounds = 10;
let population_sizes = vec![1, 2, 3, 4, 5, 10];
let max_stale_generations_options = vec![Some(100)];
let target_fitness_score_options = vec![Some(0)];
let degeneration_range_options = vec![None, Some(0.001..0.995)];
let mutates = vec![
    MutateDispatch(Mutates::Once, 0.05),
    MutateDispatch(Mutates::Once, 0.1),
    MutateDispatch(Mutates::Once, 0.2),
    MutateDispatch(Mutates::Once, 0.3),
    MutateDispatch(Mutates::Once, 0.4),
    MutateDispatch(Mutates::Once, 0.5),
];
let crossovers = vec![
    CrossoverDispatch(Crossovers::Single, true),
    CrossoverDispatch(Crossovers::Single, false),
    CrossoverDispatch(Crossovers::All, true),
    CrossoverDispatch(Crossovers::All, false),
    CrossoverDispatch(Crossovers::Range, true),
    CrossoverDispatch(Crossovers::Range, false),
    CrossoverDispatch(Crossovers::Clone, true),
    CrossoverDispatch(Crossovers::Clone, false),
];
let competes = vec![
    CompeteDispatch(Competes::Elite, 0),
    CompeteDispatch(Competes::Tournament, 2),
    CompeteDispatch(Competes::Tournament, 4),
    CompeteDispatch(Competes::Tournament, 8),
];

let genotype = BinaryGenotype::builder()
    .with_gene_size(10)
    .build()
    .unwrap();
let fitness = FitnessSimpleCount;
let evolve_builder = EvolveBuilder::new()
    .with_genotype(genotype)
    .with_fitness(fitness)
    .with_fitness_ordering(FitnessOrdering::Minimize);
let evolve_fitness_to_micro_second_factor = 1_000_000;

let config = MetaConfig::builder()
    .with_evolve_builder(evolve_builder)
    .with_evolve_fitness_to_micro_second_factor(evolve_fitness_to_micro_second_factor)
    .with_rounds(rounds)
    .with_population_sizes(population_sizes)
    .with_max_stale_generations_options(max_stale_generations_options)
    .with_target_fitness_score_options(target_fitness_score_options)
    .with_degeneration_range_options(degeneration_range_options)
    .with_mutates(mutates)
    .with_crossovers(crossovers)
    .with_competes(competes)
    .build()
    .unwrap();

let permutate = MetaPermutate::new(&config).call();
println!();
println!("{}", permutate);
```

## Tests
Run tests with `cargo test`

## Benchmarks
Implemented using criterion.
Run benchmarks with `cargo bench`

## Profiling
Implemented using criterion and pprof. find the flamegraph in: ./target/criterion/profile_evolve_binary/profile/flamegraph.svg

`cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

## TODO
* Setup prelude
* Maybe seed best_chromosome back into population after degenerate?
* Make duration stats return Duration, so we can choose sec/milli/micro afterwards.
* Make fitness/simple_sum generic
* Support genotypes with variable length (for knapsack problem). A Set type?
