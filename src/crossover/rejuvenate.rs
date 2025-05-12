use super::Crossover;
use crate::chromosome::Chromosome;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Drop non-selected parents, then clone top parents to repopulate up to target_population_size,
/// then rejuvenate selected parents to children in place. No copying of chromosomes for creating
/// the offspring itself, only for repopulating the dropped non-selected parents (smaller fraction)
/// Allowed for unique genotypes.
#[derive(Clone, Debug)]
pub struct Rejuvenate {
    pub selection_rate: f32,
}
impl Crossover for Rejuvenate {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
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

        genotype.chromosome_destructor_truncate(
            &mut state.population.chromosomes,
            selected_population_size,
        );
        genotype
            .chromosome_cloner_expand(&mut state.population.chromosomes, dropped_population_size);

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
