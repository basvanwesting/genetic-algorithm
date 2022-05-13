//! A genetic algorithm implementation for Rust.
//! Inspired by the book [Genetic Algorithms in Elixir](https://pragprog.com/titles/smgaelixir/genetic-algorithms-in-elixir/)
//!
//! There are three main elements to this approach:
//! * The [Genotype](crate::genotype) (the search space)
//! * The [Fitness](crate::fitness) function (the search goal)
//! * The [Evolve](crate::evolve::Evolve) strategy (the search strategy)
//!
//! ## Quick Usage
//!
//! ```rust
//! use genetic_algorithm::evolve::prelude::*;
//!
//! // the search space
//! let genotype = BinaryGenotype::builder() // boolean genes
//!     .with_gene_size(100)                 // 100 of them
//!     .build()
//!     .unwrap();
//!
//! println!("{}", genotype);
//!
//! // the search goal to optimize towards (maximize or minimize)
//! #[derive(Clone, Debug)]
//! pub struct CountTrue;
//! impl Fitness for CountTrue {
//!     type Genotype = BinaryGenotype;
//!     fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
//!         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
//!     }
//! }
//!
//! // the search strategy
//! let mut rng = rand::thread_rng();       // a randomness provider implementing Trait rand::Rng
//! let evolve = Evolve::builder()
//!     .with_genotype(genotype)
//!     .with_population_size(100)          // evolve with 100 chromosomes
//!     .with_target_fitness_score(100)     // goal is 100 times true in the best chromosome
//!     .with_fitness(CountTrue)            // count the number of true values in the chromosomes
//!     .with_crossover(CrossoverAll(true)) // crossover all individual genes between 2 chromosomes for offspring
//!     .with_mutate(MutateOnce(0.2))       // mutate a single gene with a 20% probability per chromosome
//!     .with_compete(CompeteElite)         // sort the chromosomes by fitness to determine crossover order
//!     .build()
//!     .unwrap()
//!     .call(&mut rng);
//!
//! println!("{}", evolve);
//! ```
//!
//! ## Examples
//!
//! * N-Queens puzzle <https://en.wikipedia.org/wiki/Eight_queens_puzzle>
//!     * See [examples/evolve_nqueens](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_nqueens.rs)
//!     * `UniqueDiscreteGenotype<u8>` with a 64x64 chess board setup
//!     * custom `NQueensFitness` fitness
//! * Knapsack problem: <https://en.wikipedia.org/wiki/Knapsack_problem>
//!     * See [examples/evolve_knapsack](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_knapsack.rs)
//!     * See [examples/permutate_knapsack](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_knapsack.rs)
//!     * `BinaryGenotype<(weight, value)>` each gene encodes presence in the knapsack
//!     * custom `KnapsackFitness(&items, weight_limit)` fitness
//! * Infinite Monkey theorem: <https://en.wikipedia.org/wiki/Infinite_monkey_theorem>
//!     * See [examples/evolve_monkeys](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_monkeys.rs)
//!     * `DiscreteGenotype<u8>` 100 monkeys randomly typing characters in a loop
//!     * custom fitness using hamming distance
//! * Custom Fitness function with LRU cache
//!     * See [examples/evolve_binary_lru_cache_fitness](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_binary_lru_cache_fitness.rs)
//!     * _Note: doesn't help performance much in this case..._
//! * Permutation strategy instead of Evolve strategy for small search spaces
//!     * See [example/permutate_binary](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_binary.rs)
//!

pub mod chromosome;
pub mod compete;
pub mod crossover;
pub mod evolve;
pub mod fitness;
pub mod genotype;
pub mod meta;
pub mod mutate;
pub mod permutate;
pub mod population;
