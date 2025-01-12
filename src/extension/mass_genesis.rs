use super::{Extension, ExtensionEvent};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use crate::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use std::time::Instant;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where only an adam
/// and eve of current best chromosomes survive
///
/// Ensure you have some population growth in select/crossover by setting the
/// [Select](crate::select::Select) selection_rate > 0.5 in order for the population to recover
#[derive(Debug, Clone)]
pub struct MassGenesis {
    pub cardinality_threshold: usize,
}

impl Extension for MassGenesis {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        _rng: &mut R,
    ) {
        if state.population.size() >= config.selected_population_size {
            let now = Instant::now();
            if let Some(cardinality) = state.population_cardinality() {
                if cardinality <= self.cardinality_threshold {
                    reporter.on_extension_event(
                        ExtensionEvent::MassGenesis("".to_string()),
                        genotype,
                        state,
                        config,
                    );
                    if let Some(best_chromosome_index) = state
                        .population
                        .best_chromosome_index(config.fitness_ordering)
                    {
                        let best_chromosome = state
                            .population
                            .chromosomes
                            .swap_remove(best_chromosome_index);
                        genotype
                            .chromosome_destructor_truncate(&mut state.population.chromosomes, 0);
                        state
                            .population
                            .chromosomes
                            .push(genotype.chromosome_cloner(&best_chromosome));
                        state.population.chromosomes.push(best_chromosome);
                    }
                }
            }
            state.add_duration(StrategyAction::Extension, now.elapsed());
        }
    }
}

impl MassGenesis {
    pub fn new(cardinality_threshold: usize) -> Self {
        Self {
            cardinality_threshold,
        }
    }
}
