use super::Crossover;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::marker::PhantomData;
use std::time::Instant;

/// Drop non-selected parents, then clone top parents to repopulate up to target_population_size,
/// then rejuvenate selected parents to children in place. No copying of chromosomes for creating
/// the offspring itself, only for repopulating the dropped non-selected parents (smaller fraction)
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Rejuvenate<G: EvolveGenotype> {
    _phantom: PhantomData<G>,
    pub selection_rate: f32,
}
impl<G: EvolveGenotype> Crossover for Rejuvenate<G> {
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
        let dropped_population_size = (existing_population_size - selected_population_size).max(0);

        state
            .population
            .chromosomes
            .truncate(selected_population_size);
        self.expand_chromosome_population(
            &mut state.population.chromosomes,
            dropped_population_size,
        );

        state
            .population
            .chromosomes
            .iter_mut()
            .take(selected_population_size)
            .for_each(|c| c.reset_age());
        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
}

impl<G: EvolveGenotype> Rejuvenate<G> {
    pub fn new(selection_rate: f32) -> Self {
        Self {
            _phantom: PhantomData,
            selection_rate,
        }
    }
}
