//! One cool thing to do with genotypes is to make a meta-genotype of all the
//! Crossover/Mutate/Compete strategies and other Evolve parameters. This could be used to optimize
//! the parameters of some other genetic algorithm. Yes, a simple nested for loop would also work,
//! but where is the fun in that? But I wasn't able to find an elegant approach to creating such a
//! heterogene setup. It was tried with Trait objects, Any and Enums, but all didn't work well:
//!
//! * Genotype wasn't allowed to become a Trait object due to it's other traits and generics.
//! * Any worked, but you still need to know all possible Genotypes up front for downcasting, defeating the flexible purpose
//! * Enum worked, but you still need to know all possible Genotypes up front for wrapping, defeating the flexible purpose
//!
//! So, after some consideration I settled on using an nested index based Genotype
//! `MultiDiscreteGenotype<usize>` indices of external vectors of arbitrary types, which should
//! then be retrieved in the fitness function. Only one type is allowed per external vector, so the
//! Crossover/Mutate/Compete strategies all have a Dispatch implementation forwarding to the
//! underlying types (e.g. `CompeteDispatch(Competes::Tournament, 4)`)
//!
//! See example meta_evolve_binary for an meta analysis of the evolution strategy:
//!
//! * See [example/meta_evolve_binary](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/meta_evolve_binary.rs) `cargo run --example meta_evolve_binary --release`
//! * See [example/meta_evolve_nqueens](https://github.com/basvanwesting/genetic-algorithm/blob/main/examples/meta_evolve_nqueens.rs) `cargo run --example meta_evolve_nqueens --release`
//!
//! Currently implemented as a permutation, but with caching an evolve strategy could also be used for larger search spaces.
//!
//! ```rust
//! use genetic_algorithm::fitness::placeholders::CountTrue;
//! use genetic_algorithm::meta::prelude::*;
//!
//! let rounds = 10;
//! let population_sizes = vec![2, 4, 8];
//! let max_stale_generations_options = vec![Some(10)];
//! let target_fitness_score_options = vec![Some(0)];
//! let degeneration_range_options = vec![None, Some(0.001..0.995)];
//! let mass_extinction_options = vec![None, Some(MassExtinction::new(0.9, 0.1))];
//! let mutates = vec![
//!     MutateDispatch(Mutates::Once, 0.05),
//!     MutateDispatch(Mutates::Once, 0.2),
//!     MutateDispatch(Mutates::Once, 0.4),
//! ];
//! let crossovers = vec![
//!     CrossoverDispatch(Crossovers::Clone, true),
//!     CrossoverDispatch(Crossovers::SingleGene, false),
//!     CrossoverDispatch(Crossovers::SingleGene, true),
//!     CrossoverDispatch(Crossovers::SinglePoint, true),
//!     CrossoverDispatch(Crossovers::Uniform, true),
//! ];
//! let competes = vec![
//!     CompeteDispatch(Competes::Elite, 0),
//!     CompeteDispatch(Competes::Tournament, 3),
//!     CompeteDispatch(Competes::Tournament, 4),
//! ];
//!
//! let genotype = BinaryGenotype::builder()
//!     .with_genes_size(10)
//!     .build()
//!     .unwrap();
//! let fitness = CountTrue;
//! let evolve_builder = EvolveBuilder::new()
//!     .with_genotype(genotype)
//!     .with_fitness(fitness)
//!     .with_fitness_ordering(FitnessOrdering::Minimize);
//! let evolve_fitness_to_micro_second_factor = 1_000_000;
//!
//! let config = MetaConfig::builder()
//!     .with_evolve_builder(evolve_builder)
//!     .with_evolve_fitness_to_micro_second_factor(evolve_fitness_to_micro_second_factor)
//!     .with_rounds(rounds)
//!     .with_population_sizes(population_sizes)
//!     .with_max_stale_generations_options(max_stale_generations_options)
//!     .with_target_fitness_score_options(target_fitness_score_options)
//!     .with_degeneration_range_options(degeneration_range_options)
//!     .with_mass_extinction_options(mass_extinction_options)
//!     .with_mutates(mutates)
//!     .with_crossovers(crossovers)
//!     .with_competes(competes)
//!     .build()
//!     .unwrap();
//!
//! let permutate = MetaPermutate::new(&config).call();
//! println!();
//! println!("{}", permutate);
//!
//! // meta-permutate population_size: 270
//!
//! // [...]
//!
//! // meta-permutate:
//! //   best_population_size: 2
//! //   best_max_stale_generations: Some(10)
//! //   best_target_fitness_score: Some(0)
//! //   best_degeneration_range: None
//! //   best_mass_extinction: None
//! //   best_mutate: Some(MutateDispatch(Random, 0.4))
//! //   best_crossover: Some(CrossoverDispatch(Clone, true))
//! //   best_compete: Some(CompeteDispatch(Elite, 0))
//! ```

mod config;
mod fitness;
mod permutate;
pub mod prelude;
mod stats;

pub use self::config::{
    Config as MetaConfig, ConfigBuilder as MetaConfigBuilder,
    TryFromConfigBuilderError as TryFromMetaConfigBuilderError,
};
pub use self::fitness::Fitness as MetaFitness;
pub use self::permutate::Permutate as MetaPermutate;
pub use self::stats::Stats as MetaStats;
