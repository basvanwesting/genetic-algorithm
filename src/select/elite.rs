use super::Select;
use crate::chromosome::Chromosome;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::prelude::*;
use std::cmp::Reverse;
use std::time::Instant;

/// Simply sort the chromosomes with fittest first. Then take the target_population_size (or full
/// population when in shortage) of the populations best and drop excess chromosomes. This approach
/// has the risk of locking in to a local optimum.
#[derive(Clone, Debug)]
pub struct Elite;

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
            config.target_population_size,
        );
        state.add_duration(StrategyAction::Select, now.elapsed());
    }
}

impl Elite {
    pub fn new() -> Self {
        Self {}
    }
}
