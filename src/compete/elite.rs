use super::Compete;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::Allele;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::prelude::*;
use std::cmp::Reverse;
use std::time::Instant;

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
    ) {
        let now = Instant::now();
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
        state
            .population
            .truncate_front(config.target_population_size);
        *state.durations.entry("compete").or_default() += now.elapsed();
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
