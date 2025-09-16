//! A genetic algorithm implementation for Rust.
//! Inspired by the book [Genetic Algorithms in Elixir](https://pragprog.com/titles/smgaelixir/genetic-algorithms-in-elixir/)
//!
//! There are three main elements to this approach:
//! * The [Genotype](crate::genotype) (the search space)
//! * The [Fitness](crate::fitness) function (the search goal)
//! * The [Strategy](crate::strategy) (the search strategy)
//!     * [Evolve](crate::strategy::evolve::Evolve) (evolution strategy)
//!     * [Permutate](crate::strategy::permutate::Permutate) (for small search spaces, with a 100% guarantee)
//!     * [HillClimb](crate::strategy::hill_climb::HillClimb) (when search space is convex with little local optima or when crossover is impossible/inefficient)
//!
//! Terminology:
//! * [Population](crate::population): a population has `population_size` number of individuals (called chromosomes).
//! * [Chromosome](crate::chromosome): a chromosome has `genes_size` number of genes
//! * [Allele](crate::genotype::Allele): alleles are the possible values of the genes
//! * Gene: a gene is a combination of position in the chromosome and value of the gene (allele)
//! * [Genes](crate::genotype::Genes): storage trait of the genes for a chromosome, mostly `Vec<Allele>` but alternatives possible
//! * [Genotype](crate::genotype): Knows how to generate, mutate and crossover chromosomes efficiently
//! * [Fitness](crate::fitness): knows how to determine the fitness of a chromosome
//!
//! All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
//!
//! ## Quick Usage
//!
//! ```rust
//! use genetic_algorithm::strategy::evolve::prelude::*;
//!
//! // the search space
//! let genotype = BinaryGenotype::builder() // boolean alleles
//!     .with_genes_size(100)                // 100 genes per chromosome
//!     .build()
//!     .unwrap();
//!
//! println!("{}", genotype);
//!
//! // the search goal to optimize towards (maximize or minimize)
//! #[derive(Clone, Debug)]
//! pub struct CountTrue;
//! impl Fitness for CountTrue {
//!     type Genotype = BinaryGenotype; // Genes = Vec<bool>
//!     fn calculate_for_chromosome(
//!         &mut self,
//!         chromosome: &FitnessChromosome<Self>,
//!         _genotype: &FitnessGenotype<Self>
//!     ) -> Option<FitnessValue> {
//!         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
//!     }
//! }
//!
//! // the search strategy
//! let evolve = Evolve::builder()
//!     .with_genotype(genotype)
//!     .with_select(SelectElite::new(0.5, 0.02))         // sort the chromosomes by fitness to determine crossover order. Strive to replace 50% of the population with offspring. Allow 2% through the non-generational best chromosomes gate before selection and replacement
//!     .with_crossover(CrossoverUniform::new(0.7, 0.8))  // crossover all individual genes between 2 chromosomes for offspring with 70% parent selection (30% do not produce offspring) and 80% chance of crossover (20% of parents just clone)
//!     .with_mutate(MutateSingleGene::new(0.2))          // mutate offspring for a single gene with a 20% probability per chromosome
//!     .with_fitness(CountTrue)                          // count the number of true values in the chromosomes
//!     .with_fitness_ordering(FitnessOrdering::Maximize) // optional, default is Maximize, aim towards the most true values
//!     .with_target_population_size(100)                 // evolve with 100 chromosomes
//!     .with_target_fitness_score(100)                   // goal is 100 times true in the best chromosome
//!     .with_reporter(EvolveReporterSimple::new(100))    // optional builder step, report every 100 generations
//!     .call()
//!     .unwrap();
//!
//! println!("{}", evolve);
//!
//! // it's all about the best genes after all
//! let (best_genes, best_fitness_score) = evolve.best_genes_and_fitness_score().unwrap();
//! assert_eq!(best_genes, vec![true; 100]);
//! assert_eq!(best_fitness_score, 100);
//! ```
//!
//! ## Tests
//!
//! Use `.with_rng_seed_from_u64(0)` builder step to create deterministic tests results.
//!
//! ## Examples
//!
//! * N-Queens puzzle <https://en.wikipedia.org/wiki/Eight_queens_puzzle>
//!     * See [examples/evolve_nqueens](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_nqueens.rs)
//!     * See [examples/hill_climb_nqueens](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/hill_climb_nqueens.rs)
//!     * `UniqueGenotype<u8>` with a 64x64 chess board setup
//!     * custom `NQueensFitness` fitness
//! * Knapsack problem: <https://en.wikipedia.org/wiki/Knapsack_problem>
//!     * See [examples/evolve_knapsack](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_knapsack.rs)
//!     * See [examples/permutate_knapsack](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_knapsack.rs)
//!     * `BinaryGenotype<Item(weight, value)>` each gene encodes presence in the knapsack
//!     * custom `KnapsackFitness(&items, weight_limit)` fitness
//! * Infinite Monkey theorem: <https://en.wikipedia.org/wiki/Infinite_monkey_theorem>
//!     * See [examples/evolve_monkeys](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_monkeys.rs)
//!     * `ListGenotype<char>` 100 monkeys randomly typing characters in a loop
//!     * custom fitness using hamming distance
//! * Permutation strategy instead of Evolve strategy for small search spaces, with a 100% guarantee
//!     * See [examples/permutate_knapsack](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_knapsack.rs)
//!     * See [examples/permutate_scrabble](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_scrabble.rs)
//! * HillClimb strategy instead of Evolve strategy, when crossover is impossible or inefficient
//!     * See [examples/hill_climb_nqueens](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/hill_climb_nqueens.rs)
//!     * See [examples/hill_climb_table_seating](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/hill_climb_table_seating.rs)
//! * Explore vector genes [BinaryGenotype](genotype::BinaryGenotype) versus other storage [BitGenotype](genotype::BitGenotype)
//!     * See [examples/evolve_bit_v_binary](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_bit_v_binary.rs)
//! * Explore internal and external multithreading options
//!     * See [examples/explore_multithreading](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/explore_multithreading.rs)
//! * Use superset StrategyBuilder for easier switching in implementation
//!     * See [examples/explore_strategies](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/explore_strategies.rs)
//! * Use fitness LRU cache
//!     * See [examples/evolve_binary_cache_fitness](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/evolve_binary_cache_fitness.rs)
//!     * _Note: doesn't help performance much in this case... or any case, better fix your population diversity_
//! * Custom Reporting implementation
//!     * See [examples/permutate_scrabble](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/permutate_scrabble.rs)
//!
//! ## Performance considerations
//!
//! For the [Evolve](strategy::evolve::Evolve) strategy:
//!
//! * [Reporting](strategy::reporter): start with [EvolveReporterSimple](strategy::evolve::EvolveReporterSimple) for basic understanding of:
//!   * fitness v. framework overhead
//!   * staleness and population characteristics (cardinality etc.)
//! * [Select](select): no considerations. All selects are basically some form of in-place
//!   sorting of some kind. This is relatively fast compared to the rest of the
//!   operations.
//! * [Crossover](crossover): the workhorse of internal parts. Crossover touches most genes each
//!   generation and clones up to the whole population to produce offspring (depending on
//!   selection-rate). It also calculates new genes hashes if enabled on the [Genotype](genotype),
//!   which has a relatively high overhead on the main Evolve loop.
//! * [Mutate](mutate): no considerations. It touches genes like crossover does, but should
//!   be used sparingly anyway; with low gene counts (<10%) and low probability (5-20%)
//! * [Fitness](fitness): can be anything. This fully depends on the user domain. Parallelize
//!   it using `with_par_fitness()` in the Builder. But beware that parallelization
//!   has it's own overhead and is not always faster.
//!
//! **GPU acceleration**
//!
//! There are two genotypes where Genes (N) and Population (M) are a stored in single contiguous
//! memory range of Alleles (T) with length N*M on the heap. A pointer to this data can be taken to
//! calculate the whole population at once. These are:
//! * [DynamicMatrixGenotype](genotype::DynamicMatrixGenotype)
//! * [StaticMatrixGenotype](genotype::StaticMatrixGenotype)
//!
//! Useful in the following strategies where a whole population is calculated:
//! * [Evolve](crate::strategy::evolve::Evolve)
//! * [HillClimb](crate::strategy::hill_climb::HillClimb)-[SteepestAscent](crate::strategy::hill_climb::HillClimbVariant::SteepestAscent)
//!
//! Possibly a GPU compatible memory layout still needs to be added. The current implementation
//! just provides all the basic building blocks to implement this. Please open a
//! [github](https://github.com/basvanwesting/genetic-algorithm) issue for further support.
pub mod allele;
pub mod chromosome;
pub mod crossover;
pub mod errors;
pub mod extension;
pub mod fitness;
pub mod genotype;
pub mod mutate;
pub mod population;
pub mod select;
pub mod strategy;
