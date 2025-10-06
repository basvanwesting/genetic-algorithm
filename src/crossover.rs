//! The crossover phase where two parent chromosomes create two children chromosomes. The
//! [selection](crate::select) phase determines the order the parent pairing (overall with fitter
//! first).
//!
//! The selection_rate is the fraction of parents which are selected for
//! reproduction. This selection adds offspring to the population, the other
//! parents do not. The population now grows by the added offspring, as the
//! parents are not replaced yet. Value should typically be between 0.4 and
//! 0.8. High values risk of premature convergence. Low values reduce diversity
//! if overused.
//!
//! The crossover_rate (or recombination-rate) is the fraction of selected parents to crossover,
//! the remaining parents just clone as offspring. Value should typically be between 0.5 and 0.8.
//! High values converge faster, but risk losing good solutions. Low values have poor exploration
//! and risk of premature convergence
//!
//! Normally the crossover adds children to the popluation, thus increasing the population_size
//! above the target_population_size. Selection will reduce this again in the next generation
mod clone;
mod multi_gene;
mod multi_point;
mod rejuvenate;
mod single_gene;
mod single_point;
mod uniform;
mod wrapper;

pub use self::clone::Clone as CrossoverClone;
pub use self::multi_gene::MultiGene as CrossoverMultiGene;
pub use self::multi_point::MultiPoint as CrossoverMultiPoint;
pub use self::rejuvenate::Rejuvenate as CrossoverRejuvenate;
pub use self::single_gene::SingleGene as CrossoverSingleGene;
pub use self::single_point::SinglePoint as CrossoverSinglePoint;
pub use self::uniform::Uniform as CrossoverUniform;
pub use self::wrapper::Wrapper as CrossoverWrapper;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;

/// This is just a shortcut for `Self::Genotype`
pub type CrossoverGenotype<C> = <C as Crossover>::Genotype;
/// This is just a shortcut for `EvolveState<Self::Genotype>,`
pub type CrossoverEvolveState<C> = EvolveState<<C as Crossover>::Genotype>;

/// # Optional Custom User implementation (rarely needed)
///
/// For the user API, the Crossover Trait has an associated Genotype. This way the user can
/// implement a specialized Crossover alterative with access to the user's Genotype specific
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
/// struct CustomCrossover {
///     pub selection_rate: f32,
/// };
/// impl Crossover for CustomCrossover {
///     type Genotype = MultiRangeGenotype<f32>;
///
///     fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
///         &mut self,
///         genotype: &Self::Genotype,
///         state: &mut EvolveState<Self::Genotype>,
///         _config: &EvolveConfig,
///         _reporter: &mut SR,
///         _rng: &mut R,
///     ) {
///         let now = Instant::now();
///         let existing_population_size = state.population.chromosomes.len();
///         let selected_population_size =
///             (existing_population_size as f32 * self.selection_rate).ceil() as usize;
///
///         // Important!!! Append offspring as recycled clones from parents (will crossover later)
///         state.population.expand_from_within(selected_population_size);
///
///         // Skip the parents, iterate over the freshly appended offspring
///         let iterator = state
///             .population
///             .chromosomes
///             .iter_mut()
///             .skip(existing_population_size);
///
///         // Crossover the offspring clones
///         for (offspring1, offspring2) in iterator.tuples() {
///             // Custom logic, for instance, swap all genes with even index
///             for even_index in (0..genotype.genes_size()).filter(|x| x % 2 == 0) {
///                 std::mem::swap(&mut offspring1.genes[even_index], &mut offspring2.genes[even_index]);
///                 // MultiRangeGenotype specific methods are available if needed
///             }
///             // Important!!! Remember to reset the chromosome metadata after manipulation
///             offspring1.reset_metadata(genotype.genes_hashing);
///             offspring2.reset_metadata(genotype.genes_hashing);
///         }
///         // Optionally, keep track of duration for reporting
///         state.add_duration(StrategyAction::Crossover, now.elapsed());
///     }
/// }
/// ```
pub trait Crossover: Clone + Send + Sync + std::fmt::Debug {
    type Genotype: EvolveGenotype;

    fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_indexes(&self) -> bool {
        false
    }
    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_points(&self) -> bool {
        false
    }
}
