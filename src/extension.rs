//! When approacking a (local) optimum in the fitness score, the variation in the population goes
//! down dramatically. The offspring will become clones of the parents and the only factor seeding
//! randomness is the mutation of the offspring. But this remaining randomness might not be
//! selected for, killing of the offspring again. This reduces the efficiency, but also has the
//! risk of local optimum lock-in. To increase the variation in the population, an
//! [extension](crate::extension) mechanisms can optionally be used
mod mass_deduplication;
mod mass_degeneration;
mod mass_extinction;
mod mass_genesis;

use crate::chromosome::Chromosome;
mod noop;
mod wrapper;

pub use self::mass_deduplication::MassDeduplication as ExtensionMassDeduplication;
pub use self::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use self::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use self::noop::Noop as ExtensionNoop;
pub use self::wrapper::Wrapper as ExtensionWrapper;

use crate::genotype::{EvolveGenotype, Genotype};
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;

/// This is just a shortcut for `Self::Genotype`
pub type ExtensionGenotype<E> = <E as Extension>::Genotype;
/// This is just a shortcut for `EvolveState<Self::Genotype>,`
pub type ExtensionEvolveState<E> = EvolveState<<E as Extension>::Genotype>;
/// This is just a shortcut
pub type ExtensionAllele<E> = <<E as Extension>::Genotype as Genotype>::Allele;

/// # Optional Custom User implementation (rarely needed)
///
/// For the user API, the Extension Trait has an associated Genotype. This way the user can
/// implement a specialized Extension alterative with access to the user's Genotype specific
/// methods at hand.
///
/// # Example
/// ```rust
/// use genetic_algorithm::strategy::evolve::prelude::*;
/// use std::time::Instant;
/// use itertools::Itertools;
/// use rand::Rng;
///
/// #[derive(Clone, Debug)]
/// pub struct CustomExtension {
///     pub cardinality_threshold: usize,
/// }
///
/// impl Extension for CustomExtension {
///     type Genotype = MultiRangeGenotype<f32>;
///
///     fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
///         &mut self,
///         genotype: &Self::Genotype,
///         state: &mut EvolveState<Self::Genotype>,
///         config: &EvolveConfig,
///         reporter: &mut SR,
///         _rng: &mut R,
///     ) {
///         if genotype.genes_hashing() && state.population.size() >= config.target_population_size {
///             let now = Instant::now();
///             if let Some(cardinality) = state.population_cardinality() {
///                 if cardinality <= self.cardinality_threshold {
///                     reporter.on_extension_event(
///                         ExtensionEvent::Custom("make unique".to_string()),
///                         genotype,
///                         state,
///                         config,
///                     );
///
///                     let mut unique_chromosomes =
///                         self.extract_unique_chromosomes(genotype, state, config);
///                     let unique_size = unique_chromosomes.len();
///
///                     let remaining_size = 2usize.saturating_sub(unique_size);
///                     state.population.truncate(remaining_size);
///                     state.population.chromosomes.append(&mut unique_chromosomes);
///                 }
///             }
///             state.add_duration(StrategyAction::Extension, now.elapsed());
///         }
///     }
/// }
/// ```
pub trait Extension: Clone + Send + Sync + std::fmt::Debug {
    type Genotype: EvolveGenotype;

    fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

    fn extract_elite_chromosomes(
        &self,
        _genotype: &Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        config: &EvolveConfig,
        elitism_size: usize,
    ) -> Vec<Chromosome<ExtensionAllele<Self>>> {
        let mut elite_chromosomes: Vec<Chromosome<ExtensionAllele<Self>>> =
            Vec::with_capacity(elitism_size);
        for index in state
            .population
            .best_chromosome_indices(elitism_size, config.fitness_ordering)
            .into_iter()
            .rev()
        {
            let chromosome = state.population.chromosomes.swap_remove(index);
            elite_chromosomes.push(chromosome);
        }
        elite_chromosomes
    }

    fn extract_unique_elite_chromosomes(
        &self,
        _genotype: &Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        config: &EvolveConfig,
        elitism_size: usize,
    ) -> Vec<Chromosome<ExtensionAllele<Self>>> {
        let mut elite_chromosomes: Vec<Chromosome<ExtensionAllele<Self>>> =
            Vec::with_capacity(elitism_size);
        for index in state
            .population
            .best_unique_chromosome_indices(elitism_size, config.fitness_ordering)
            .into_iter()
            .rev()
        {
            let chromosome = state.population.chromosomes.swap_remove(index);
            elite_chromosomes.push(chromosome);
        }
        elite_chromosomes
    }

    fn extract_unique_chromosomes(
        &self,
        _genotype: &Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
    ) -> Vec<Chromosome<ExtensionAllele<Self>>> {
        let mut unique_chromosomes: Vec<Chromosome<ExtensionAllele<Self>>> = Vec::new();
        for index in state
            .population
            .unique_chromosome_indices()
            .into_iter()
            .rev()
        {
            let chromosome = state.population.chromosomes.swap_remove(index);
            unique_chromosomes.push(chromosome);
        }
        unique_chromosomes
    }
}

#[derive(Clone, Debug)]
pub enum ExtensionEvent {
    MassDeduplication(String),
    MassDegeneration(String),
    MassExtinction(String),
    MassGenesis(String),
    Custom(String),
}
