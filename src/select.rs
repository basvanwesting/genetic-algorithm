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

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::prelude::*;

pub trait Select: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

    fn extract_elite_chromosomes<G: EvolveGenotype>(
        &self,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        elitism_rate: f32,
    ) -> Vec<G::Chromosome> {
        let elitism_size = ((state.population.size() as f32 * elitism_rate).ceil() as usize)
            .min(state.population.size());

        let mut elite_chromosomes: Vec<G::Chromosome> = Vec::with_capacity(elitism_size);
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

    fn survival_sizes(
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
