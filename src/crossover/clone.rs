use super::Crossover;
use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Children are clones of the parents.
/// The population is restored towards the target_population_size by keeping the best parents
/// alive. Excess parents are dropped.
///
/// Allowed for unique genotypes.
#[derive(Clone, Debug, Default)]
pub struct Clone;
impl Crossover for Clone {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        let now = Instant::now();
        let crossover_size = self.prepare_population(genotype, state, config);
        state
            .population
            .chromosomes
            .iter_mut()
            .take(crossover_size)
            .for_each(|c| c.reset_age());

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
}

impl Clone {
    pub fn new() -> Self {
        Self
    }
}
