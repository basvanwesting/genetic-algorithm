# genetic-algorithm
A genetic algorithm implementation for Rust.
Inspired by the book [Genetic Algorithms in Elixir](https://pragprog.com/titles/smgaelixir/genetic-algorithms-in-elixir/)

There are three main elements to this approach:
* The Genotype (the search space)
* The Fitness function (the search goal)
* The Evolve strategy (the search strategy)

Terminology:
* Population: a population has `population_size` number of individuals (called chromosomes).
* Chromosome: a chromosome has `genes_size` number of genes
* Gene: a gene is a combination of position in the chromosome and value of the gene (allele)
* Allele: alleles are the possible values of the genes
* Genotype: holds the `genes_size` and alleles and knows how to generate and mutate chromosomes efficiently
* Fitness: knows how to determine the fitness of a chromosome

## Documentation

See [docs.rs](https://docs.rs/genetic_algorithm/latest/genetic_algorithm)

## Quick Usage

```rust
use genetic_algorithm::evolve::prelude::*;

// the search space
let genotype = BinaryGenotype::builder() // boolean alleles
    .with_genes_size(100)                // 100 genes per chromosome
    .build()
    .unwrap();

println!("{}", genotype);

// the search goal to optimize towards (maximize or minimize)
#[derive(Clone, Debug)]
pub struct CountTrue;
impl Fitness for CountTrue {
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}

// the search strategy
let mut rng = rand::thread_rng();     // a randomness provider implementing Trait rand::Rng
let evolve = Evolve::builder()
    .with_genotype(genotype)
    .with_population_size(100)        // evolve with 100 chromosomes
    .with_target_fitness_score(100)   // goal is 100 times true in the best chromosome
    .with_fitness(CountTrue)          // count the number of true values in the chromosomes
    .with_crossover(CrossoverUniform(true)) // crossover all individual genes between 2 chromosomes for offspring
    .with_mutate(MutateOnce(0.2))     // mutate a single gene with a 20% probability per chromosome
    .with_compete(CompeteElite)       // sort the chromosomes by fitness to determine crossover order
    .call(&mut rng);
    .unwrap()

println!("{}", evolve);
```

## Examples
Run with `cargo run --example [EXAMPLE_BASENAME] --release`

* N-Queens puzzle https://en.wikipedia.org/wiki/Eight_queens_puzzle.
    * See [examples/evolve_nqueens](../main/examples/evolve_nqueens.rs)
    * `UniqueDiscreteGenotype<u8>` with a 64x64 chess board setup
    * custom `NQueensFitness` fitness
* Knapsack problem: https://en.wikipedia.org/wiki/Knapsack_problem
    * See [examples/evolve_knapsack](../main/examples/evolve_knapsack.rs)
    * See [examples/permutate_knapsack](../main/examples/permutate_knapsack.rs)
    * `BinaryGenotype<(weight, value)>` each gene encodes presence in the knapsack
    * custom `KnapsackFitness(&items, weight_limit)` fitness
* Infinite Monkey theorem: https://en.wikipedia.org/wiki/Infinite_monkey_theorem
    * See [examples/evolve_monkeys](../main/examples/evolve_monkeys.rs)
    * `DiscreteGenotype<char>` 100 monkeys randomly typing characters in a loop
    * custom fitness using hamming distance
* Custom Fitness function with LRU cache
    * See [examples/evolve_binary_lru_cache_fitness](../main/examples/evolve_binary_lru_cache_fitness.rs)
    * _Note: doesn't help performance much in this case..._
* Permutation strategy instead of Evolve strategy for small search spaces
    * See [examples/permutate_binary](../main/examples/permutate_binary.rs)


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
* Does Fitness need an associated trait for Genotype? Can this be made more lightweight?

## MAYBE
* Add Tournament competition with duplicates (needs cloning)
* Add Roulette competition with and without duplicates (with fitness ordering)
* Add OrderOne crossover for UniqueDiscreteGenotype?
* Add WholeArithmetic crossover for ContinuousGenotype?
* Rename Continuous to Real?
