use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Children are clones of the parents, effectively doubling the population if you keep all the
/// parents. Acts as no-op if no percentage of parents is kept (age is reset).
///
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Clone {
    pub parent_survival_rate: f32,
}
impl Crossover for Clone {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        let now = Instant::now();
        let population_size = state.population.size();
        let parent_survivors = std::cmp::min(
            (population_size as f32 * self.parent_survival_rate) as usize,
            population_size,
        );
        state
            .population
            .chromosomes
            .extend_from_within(..parent_survivors);
        state
            .population
            .chromosomes
            .iter_mut()
            .take(population_size)
            .for_each(|c| c.age = 0);

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
}

impl Clone {
    pub fn new(parent_survival_rate: f32) -> Self {
        Self {
            parent_survival_rate,
        }
    }
}
