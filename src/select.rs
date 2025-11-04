//! The selection phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase, dropping the chromosomes outside of the
//! target_population_size.
//!
//! The replacement_rate is the target fraction of the population which exists of
//! children. Generational Replacement and Steady-State Replacement can both be
//! modelled with this parameter by setting it respectively to 1.0 and 0.2-0.8.
//! High values converge faster, but risk losing good solutions. Low values
//! convergence slower. If there is a shortage of population after the ideal
//! fraction, firstly remaining non-selected children and secondly remaining
//! non-selected parents will be used to fill the shortage to avoid population
//! collapse.
//!
//! The elitism_rate is a non-generational elite gate, which ensures passing of the
//! best chromosomes before selection and replacement takes place. Value should
//! typically be very low, between 0.01 and 0.05. Relevant for
//! `SelectTournament` where the best chromosome is not guaranteed to be
//! selected for a tournament if the `population_size` is larger than the
//! `target_population_size`
mod elite;
mod tournament;
mod wrapper;

pub use self::elite::Elite as SelectElite;
pub use self::tournament::Tournament as SelectTournament;
pub use self::wrapper::Wrapper as SelectWrapper;

use crate::chromosome::Chromosome;

use crate::genotype::{EvolveGenotype, Genotype};
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::prelude::*;

/// This is just a shortcut for `Self::Genotype`
pub type SelectGenotype<S> = <S as Select>::Genotype;
/// This is just a shortcut for `EvolveState<Self::Genotype>,`
pub type SelectEvolveState<S> = EvolveState<<S as Select>::Genotype>;
/// This is just a shortcut
pub type SelectAllele<S> = <<S as Select>::Genotype as Genotype>::Allele;

/// # Optional Custom User implementation (rarely needed)
///
/// For the user API, the Select Trait has an associated Genotype. This way the user can implement
/// a specialized Select alterative with access to the user's Genotype specific methods at hand.
///
/// # Example
/// ```rust
/// use genetic_algorithm::strategy::evolve::prelude::*;
/// use std::time::Instant;
/// use std::cmp::Reverse;
/// use rand::Rng;
///
/// #[derive(Clone, Debug)]
/// struct CustomSelect; // or with fields
/// impl Select for CustomSelect {
///     type Genotype = MultiRangeGenotype<f32>;
///
///     fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
///         &mut self,
///         _genotype: &Self::Genotype,
///         state: &mut EvolveState<Self::Genotype>,
///         config: &EvolveConfig,
///         _reporter: &mut SR,
///         rng: &mut R,
///     ) {
///         let now = Instant::now();
///
///         // super simple sort, no further considerations
///         match config.fitness_ordering {
///             FitnessOrdering::Maximize => {
///                 state.population.chromosomes.sort_unstable_by_key(|c| match c.fitness_score() {
///                     Some(fitness_score) => Reverse(fitness_score),
///                     None => Reverse(FitnessValue::MIN),
///                 });
///             }
///             FitnessOrdering::Minimize => {
///                 state.population.chromosomes.sort_unstable_by_key(|c| match c.fitness_score() {
///                     Some(fitness_score) => fitness_score,
///                     None => FitnessValue::MAX,
///                 });
///             }
///         }
///
///         // Optionally, keep track of duration for reporting
///         state.add_duration(StrategyAction::Select, now.elapsed());
///     }
/// }
/// ```
pub trait Select: Clone + Send + Sync + std::fmt::Debug {
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
        state: &mut EvolveState<Self::Genotype>,
        config: &EvolveConfig,
        elitism_rate: f32,
    ) -> Vec<Chromosome<SelectAllele<Self>>> {
        let elitism_size = ((state.population.size() as f32 * elitism_rate).ceil() as usize)
            .min(state.population.size());

        let mut elite_chromosomes: Vec<Chromosome<SelectAllele<Self>>> =
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

    fn parent_and_offspring_survival_sizes(
        &self,
        parents_size: usize,
        offspring_size: usize,
        target_size: usize,
        replacement_rate: f32,
    ) -> (usize, usize) {
        let target_offspring_size =
            ((target_size as f32 * replacement_rate).ceil() as usize).min(target_size);
        let target_parents_size = target_size - target_offspring_size;

        let mut new_offspring_size = target_offspring_size.min(offspring_size);
        let mut new_parents_size = target_parents_size.min(parents_size);

        let target_shortage =
            (target_size as isize - new_offspring_size as isize - new_parents_size as isize).max(0)
                as usize;
        new_offspring_size += target_shortage.min(offspring_size - new_offspring_size);

        let target_shortage =
            (target_size as isize - new_offspring_size as isize - new_parents_size as isize).max(0)
                as usize;
        new_parents_size += target_shortage.min(parents_size - new_parents_size);

        (new_parents_size, new_offspring_size)
    }
}
