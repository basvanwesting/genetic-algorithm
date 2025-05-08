//! The selection phase, where chromosomes are lined up for pairing in the
//! [crossover](crate::crossover) phase, dropping the chromosomes outside of the
//! target_population_size.
mod elite;
mod tournament;
mod wrapper;

pub use self::elite::Elite as SelectElite;
pub use self::tournament::Tournament as SelectTournament;
pub use self::wrapper::Wrapper as SelectWrapper;

use crate::chromosome::Chromosome;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::prelude::*;
use std::cmp::Reverse;

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
        let mut elite_chromosomes: Vec<G::Chromosome> = Vec::new(); //small capacity
        if elitism_rate > 0.0 {
            let elitism_size = ((state.population.size() as f32 * elitism_rate).ceil() as usize)
                .min(state.population.size());

            match config.fitness_ordering {
                FitnessOrdering::Maximize => {
                    state
                        .population
                        .chromosomes
                        .sort_unstable_by_key(|c| match c.fitness_score() {
                            Some(fitness_score) => Reverse(fitness_score),
                            None => Reverse(FitnessValue::MIN),
                        })
                }
                FitnessOrdering::Minimize => {
                    state
                        .population
                        .chromosomes
                        .sort_unstable_by_key(|c| match c.fitness_score() {
                            Some(fitness_score) => fitness_score,
                            None => FitnessValue::MAX,
                        })
                }
            }

            for index in (0..elitism_size).rev() {
                let chromosome = state.population.chromosomes.swap_remove(index);
                elite_chromosomes.push(chromosome);
            }
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
