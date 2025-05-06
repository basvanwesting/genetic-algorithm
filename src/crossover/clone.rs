use super::Crossover;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Children are clones of the parents.
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Clone {
    pub selection_rate: f32,
}
impl Crossover for Clone {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        let now = Instant::now();
        let selected_population_size =
            (state.population.size() as f32 * self.selection_rate).ceil() as usize;
        genotype
            .chromosome_cloner_expand(&mut state.population.chromosomes, selected_population_size);
        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
}

impl Clone {
    pub fn new(selection_rate: f32) -> Self {
        Self { selection_rate }
    }
}
