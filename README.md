# genetic-algorithm
A genetic algorithm implementation for Rust.
Inspired by the book [Genetic Algorithms in Elixir](https://pragprog.com/titles/smgaelixir/genetic-algorithms-in-elixir/)

There are three main elements to this approach:
* The Genotype (the search space)
* The Fitness function (the search goal)
* The Evolve strategy (the search strategy)

## Documentation

See [docs.rs](https://docs.rs/genetic_algorithm/latest/genetic_algorithm)

## Quick Usage

```rust
use genetic_algorithm::evolve::prelude::*;

// the search space
let genotype = BinaryGenotype::builder() // boolean genes
    .with_gene_size(100)                 // 100 of them
    .build()
    .unwrap();

println!("{}", genotype);

// the search goal to optimize towards (maximize or minimize)
#[derive(Clone, Debug)]
pub struct CountTrue;
impl Fitness for CountTrue {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}

// the search strategy
let mut rng = rand::thread_rng();       // a randomness provider implementing Trait rand::Rng
let evolve = Evolve::builder()
    .with_genotype(genotype)
    .with_population_size(100)          // evolve with 100 chromosomes
    .with_target_fitness_score(100)     // goal is 100 times true in the best chromosome
    .with_fitness(CountTrue)            // count the number of true values in the chromosomes
    .with_crossover(CrossoverAll(true)) // crossover all individual genes between 2 chromosomes for offspring
    .with_mutate(MutateOnce(0.2))       // mutate a single gene with a 20% probability per chromosome
    .with_compete(CompeteElite)         // sort the chromosomes by fitness to determine crossover order
    .build()
    .unwrap()
    .call(&mut rng);

println!("{}", evolve);
```

## Examples
Run with `cargo run --example [EXAMPLE_BASENAME] --release`

* N-Queens puzzle https://en.wikipedia.org/wiki/Eight_queens_puzzle.
    * See [example/evolve_nqueens](../main/examples/evolve_nqueens.rs)
    * `UniqueDiscreteGenotype<u8>` with a 64x64 chess board setup and custom `NQueensFitness` fitness
* Knapsack problem: https://en.wikipedia.org/wiki/Knapsack_problem
    * See [example/evolve_knapsack](../main/examples/evolve_knapsack.rs)
    * `DiscreteGenotype<(weight, value)>` with a custom `KnapsackFitness(weight_limit)` fitness
* Infinite Monkey theorem: https://en.wikipedia.org/wiki/Infinite_monkey_theorem
    * See [example/evolve_monkeys](../main/examples/evolve_monkeys.rs)
    * `DiscreteGenotype<u8>` 100 monkeys randomly typing characters in a loop
* Custom Fitness function with LRU cache
    * See [example/evolve_binary_lru_cache_fitness](../main/examples/evolve_binary_lru_cache_fitness.rs)
    * Note: doesn't help performance much in this case...
* Permutation strategy instead of Evolve strategy for small search spaces
    * See [example/permutate_binary](../main/examples/permutate_binary.rs)


## Tests
Run tests with `cargo test`

## Benchmarks
Implemented using criterion. Run benchmarks with `cargo bench`

## Profiling
Implemented using criterion and pprof.

Find the flamegraph in: `./target/criterion/profile_evolve_binary/profile/flamegraph.svg`

Run with `cargo run --example profile_evolve_binary --release -- --bench --profile-time 5`

## TODO
* Maybe seed best_chromosome back into population after degenerate?
* Make duration stats return Duration, so we can choose sec/milli/micro afterwards.
* Make fitness/simple_sum generic
* Support genotypes with variable length (for knapsack problem). A Bag / Set type?
* Add a chromosome stream for Permutate, instead of initializing the full population
