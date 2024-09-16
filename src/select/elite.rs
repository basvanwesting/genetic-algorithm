use super::Select;
use crate::chromosome::Chromosome;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::prelude::*;
use std::cmp::Reverse;
use std::time::Instant;

/// Simply sort the chromosomes with fittest first. Then take the selection_rate of the populations
/// best and drop excess chromosomes. This approach has the risk of locking in to a local optimum.
#[derive(Clone, Debug)]
pub struct Elite {
    pub selection_rate: f32,
}

impl Select for Elite {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        let now = Instant::now();
        let population_size = state.population.size();
        let selected_population_size = ((population_size as f32 * self.selection_rate).ceil()
            as usize)
            .min(population_size)
            .max(2);

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
        genotype.chromosome_destructor_truncate(
            &mut state.population.chromosomes,
            selected_population_size,
        );
        state.add_duration(StrategyAction::Select, now.elapsed());
    }
}

impl Elite {
    pub fn new(selection_rate: f32) -> Self {
        Self { selection_rate }
    }
}
