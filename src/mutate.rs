//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
mod multi_gene;
mod multi_gene_dynamic;
mod multi_gene_range;
mod single_gene;
mod single_gene_dynamic;
mod wrapper;

pub use self::multi_gene::MultiGene as MutateMultiGene;
pub use self::multi_gene_dynamic::MultiGeneDynamic as MutateMultiGeneDynamic;
pub use self::multi_gene_range::MultiGeneRange as MutateMultiGeneRange;
pub use self::single_gene::SingleGene as MutateSingleGene;
pub use self::single_gene_dynamic::SingleGeneDynamic as MutateSingleGeneDynamic;
pub use self::wrapper::Wrapper as MutateWrapper;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;

/// This is just a shortcut for `Self::Genotype`
pub type MutateGenotype<M> = <M as Mutate>::Genotype;
/// This is just a shortcut for `EvolveState<Self::Genotype>,`
pub type MutateEvolveState<M> = EvolveState<<M as Mutate>::Genotype>;

/// # Optional Custom User implementation (rarely needed)
///
/// For the user API, the Mutate Trait has an associated Genotype. This way the user can implement
/// a specialized Mutate alterative with access to the user's Genotype specific methods at hand.
///
/// # Example
/// ```rust
/// use genetic_algorithm::strategy::evolve::prelude::*;
/// use std::time::Instant;
/// use rand::Rng;
///
/// #[derive(Clone, Debug)]
/// struct CustomMutate; // or with fields
/// impl Mutate for CustomMutate {
///     type Genotype = MultiRangeGenotype<f32>;
///
///     fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
///         &mut self,
///         genotype: &Self::Genotype,
///         state: &mut EvolveState<Self::Genotype>,
///         _config: &EvolveConfig,
///         _reporter: &mut SR,
///         rng: &mut R,
///     ) {
///         let now = Instant::now();
///
///         // Skip the parents, iterate over the freshly crossovered offspring
///         for chromosome in state
///             .population
///             .chromosomes
///             .iter_mut()
///             .filter(|c| c.is_offspring())
///         {
///             // Custom logic, for instance mutate all genes with even index be a relative change
///             for even_index in (0..genotype.genes_size()).filter(|x| x % 2 == 0) {
///                 let delta = rng.gen_range(-1.0..=1.0);
///                 // MultiRangeGenotype specific methods are available (this one does allele bounds checking)
///                 genotype.apply_gene_delta(chromosome, even_index, delta);
///             }
///             // Important!!! Remember to reset the chromosome metadata after manipulation
///             chromosome.reset_state();
///         }
///         // Optionally, keep track of duration for reporting
///         state.add_duration(StrategyAction::Mutate, now.elapsed());
///     }
/// }
/// ```
pub trait Mutate: Clone + Send + Sync + std::fmt::Debug {
    type Genotype: EvolveGenotype;

    fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );
}

#[derive(Clone, Debug)]
pub enum MutateEvent {
    ChangeMutationProbability(String),
}
