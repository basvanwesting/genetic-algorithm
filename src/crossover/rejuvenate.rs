use super::Crossover;
use crate::chromosome::Chromosome;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Rejuvenate parents to children in place, no copying of chromosomes
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Rejuvenate {
    pub selection_rate: f32,
}
impl Crossover for Rejuvenate {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        _genotype: &mut G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        let now = Instant::now();
        let selected_population_size =
            (state.population.size() as f32 * self.selection_rate).ceil() as usize;
        state
            .population
            .chromosomes
            .iter_mut()
            .take(selected_population_size)
            .for_each(|c| c.reset_age());
        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
}

impl Rejuvenate {
    pub fn new(selection_rate: f32) -> Self {
        Self { selection_rate }
    }
}
