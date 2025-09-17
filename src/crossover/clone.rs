use super::Crossover;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::marker::PhantomData;
use std::time::Instant;

/// Children are clones of the parents.
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Clone<G: EvolveGenotype> {
    _phantom: PhantomData<G>,
    pub selection_rate: f32,
}
impl<G: EvolveGenotype> Crossover for Clone<G> {
    type Genotype = G;

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        let now = Instant::now();
        let existing_population_size = state.population.chromosomes.len();
        let selected_population_size =
            (existing_population_size as f32 * self.selection_rate).ceil() as usize;
        state
            .population
            .expand_with_recycling(selected_population_size);
        state
            .population
            .chromosomes
            .iter_mut()
            .skip(existing_population_size)
            .for_each(|c| c.reset_age());
        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
}

impl<G: EvolveGenotype> Clone<G> {
    pub fn new(selection_rate: f32) -> Self {
        Self {
            _phantom: PhantomData,
            selection_rate,
        }
    }
}
