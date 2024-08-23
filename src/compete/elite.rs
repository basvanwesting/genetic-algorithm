use super::Compete;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::Allele;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::prelude::*;
use std::cmp::Reverse;

/// Simply sort the chromosomes with fittest first. This approach has the risk of locking in to a local optimum.
///
/// Excess chromosomes, beyond the target_population_size, are dropped.
#[derive(Clone, Debug)]
pub struct Elite;
impl Compete for Elite {
    fn call<A: Allele, R: Rng, SR: EvolveReporter<Allele = A>>(
        &mut self,
        state: &mut EvolveState<A>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
        _par: bool,
    ) {
        match config.fitness_ordering {
            FitnessOrdering::Maximize => state
                .population
                .chromosomes
                .sort_unstable_by_key(|c| c.fitness_score),
            FitnessOrdering::Minimize => {
                state
                    .population
                    .chromosomes
                    .sort_unstable_by_key(|c| match c.fitness_score {
                        Some(fitness_score) => Reverse(fitness_score),
                        None => Reverse(FitnessValue::MAX),
                    })
            }
        }
        if state.population.size() > config.target_population_size {
            let to_drain_from_first = state.population.size() - config.target_population_size;
            state.population.chromosomes.drain(..to_drain_from_first);
        }
    }
}

impl Elite {
    pub fn new() -> Self {
        Self
    }
}
impl Default for Elite {
    fn default() -> Self {
        Self::new()
    }
}
