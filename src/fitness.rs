//! The search goal to optimize towards (maximize or minimize).
//!
//! Each problem will usually have its own specific [Fitness] function, therefore you need to
//! implement it yourself. Because the [Fitness] function is specific, it is also bound to the
//! [Genotype] through a trait attribute (no reason to make it generic, as the client implements for
//! a single [Genotype] type).
//!
//! See [Fitness] Trait for examples and further documentation
pub mod placeholders;
pub mod prelude;

use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::{StrategyAction, StrategyConfig, StrategyState};
use std::time::Instant;

/// Use isize for easy handling of scores (ordering, comparing) as floats are tricky in that regard.
pub type FitnessValue = isize;

#[derive(Copy, Clone, Debug)]
pub enum FitnessOrdering {
    Maximize,
    Minimize,
}

/// This is just a shortcut for `Self::Genotype`
pub type FitnessGenotype<F> = <F as Fitness>::Genotype;
/// This is just a shortcut for `<Self::Genotype as Genotype>::Genes`
pub type FitnessGenes<F> = <<F as Fitness>::Genotype as Genotype>::Genes;

/// The fitness function, is implemented as a fitness method object.
///
/// Normally the fitness returns [`Some(FitnessValue)`](FitnessValue) for the chromosome, which can be minimized or
/// maximized in the search strategy (e.g. [Evolve](crate::strategy::evolve::Evolve)) by providing the [FitnessOrdering].
///
/// If the fitness returns `None`, the chromosome is assumed invalid and taken last in the [selection](crate::select) phase (also when minimizing).
///
/// # User implementation
///
/// In the centralized track, you must implement:
/// * [`calculate_for_population(...) -> Vec<Option<FitnessValue>>`](Fitness::calculate_for_population)
///   * Designed for matrix Genotypes with possible GPU acceleration
///   * Centralized [Genotype]s use [GenesPointer](crate::chromosome::GenesPointer) chromosomes. These
///     chromosomes don't have a `genes` field to read, but a `row_id`. The matrix [Genotype] has a contiguous
///     memory `data` field with all the data, which can be calculated in one go.
///     * [DynamicRangeGenotype](crate::genotype::DynamicRangeGenotype)
///     * [StaticRangeGenotype](crate::genotype::StaticRangeGenotype)
///   * The order and length of the rows in the genotype data matrix needs to be preserved in the
///     returned vector as it matches the row_id on the chromosome
///   * The order and length of the population does not matter at all and will most likely not align.
///     The population is provided, because the full genotype matrix data also contains untainted
///     chromosomes which already have a fitness_score (and will not be updated). The results for
///     these chromosomes will be ignored, thus these do not have to be recalculated, so knowing
///     which ones might be relevant (or not). The order of the results still need to align, so if the
///     calculation is skipped, a `None` value should be inserted in the results to keep the order
///     and length aligned.
///
/// All centralized strategies use population level calculations:
/// * [Evolve](crate::strategy::evolve::Evolve)
/// * [HillClimb](crate::strategy::hill_climb::HillClimb) with [SteepestAscent](crate::strategy::hill_climb::HillClimbVariant::SteepestAscent)
/// * [Permutate](crate::strategy::permutate::Permutate) (using population windows)
///
/// # Example (calculate_for_population, static matrix calculation, GenesPointer chromosome):
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
/// use genetic_algorithm::strategy::evolve::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct SumStaticRangeGenes;
/// impl Fitness for SumStaticRangeGenes {
///     type Genotype = StaticRangeGenotype<u16, 10, 100>;
///     fn calculate_for_population(
///         &mut self,
///         population: &Population,
///         genotype: &FitnessGenotype<Self>,
///     ) -> Vec<Option<FitnessValue>> {
///         // pure matrix data calculation on [[T; N] M]
///         // the order of the rows needs to be preserved as it matches the row_id on the chromosome
///         // the order of the population does not matter at all and will most likely not align
///         genotype
///             .data
///             .iter()
///             .map(|genes| {
///                 genes
///                     .iter()
///                     .sum::<u16>() as FitnessValue
///             })
///             .map(Some)
///             .collect()
///     }
/// }
/// ```
///
/// # Example (calculate_for_population, dynamic matrix calculation, GenesPointer chromosome):
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
/// use genetic_algorithm::strategy::evolve::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct SumDynamicRangeGenes;
/// impl Fitness for SumDynamicRangeGenes {
///     type Genotype = DynamicRangeGenotype<u16>;
///     fn calculate_for_population(
///         &mut self,
///         population: &Population,
///         genotype: &FitnessGenotype<Self>,
///     ) -> Vec<Option<FitnessValue>> {
///         // pure matrix data calculation on Vec<T> with genes_size step
///         // the order of the rows needs to be preserved as it matches the row_id on the chromosome
///         // the order of the population does not matter at all and will most likely not align
///         genotype
///             .data
///             .chunks(genotype.genes_size())
///             .map(|genes| {
///                 genes
///                     .iter()
///                     .sum::<u16>() as FitnessValue
///             })
///             .map(Some)
///             .collect()
///     }
/// }
/// ```
pub trait Fitness: Clone + Send + Sync + std::fmt::Debug {
    type Genotype: Genotype;
    fn call_for_state_population<S: StrategyState, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &mut S,
        _config: &C,
    ) {
        let now = Instant::now();
        self.call_for_population(state.population_as_mut(), genotype);
        state.add_duration(StrategyAction::Fitness, now.elapsed());
    }
    fn call_for_population(&mut self, population: &mut Population, genotype: &Self::Genotype) {
        let fitness_scores = self.calculate_for_population(population, genotype);
        genotype.update_population_fitness_scores(population, fitness_scores);
    }
    /// Mandatory interception point for client implementation.
    ///
    /// The order and length of the results need to align with the order and length of the genotype data matrix.
    /// The order and length of the population does not matter at all and will most likely not align.
    fn calculate_for_population(
        &mut self,
        _population: &Population,
        _genotype: &Self::Genotype,
    ) -> Vec<Option<FitnessValue>>;
}
