use super::{Extension, ExtensionEvent};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Simulates a cambrian explosion. The controlling metric is fitness score cardinality in the
/// population after selection. When this cardinality drops to the threshold, the population is
/// randomly reduced regardless of fitness using the survival_rate (fraction of population).
///
/// Ensure you have some population growth in select/crossover by setting the
/// [Select](crate::select::Select) selection_rate > 0.5 in order for the population to recover
#[derive(Debug, Clone)]
pub struct MassExtinction {
    pub cardinality_threshold: usize,
    pub survival_rate: f32,
}

impl Extension for MassExtinction {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        if state.population.size() >= config.selected_population_size {
            let now = Instant::now();
            if let Some(cardinality) = state.population_cardinality() {
                if cardinality <= self.cardinality_threshold {
                    reporter.on_extension_event(
                        ExtensionEvent::MassExtinction("".to_string()),
                        genotype,
                        state,
                        config,
                    );

                    let remaining_size: usize = std::cmp::max(
                        (state.population.size() as f32 * self.survival_rate).ceil() as usize,
                        2,
                    );
                    state.population.shuffle(rng);
                    genotype.chromosome_destructor_truncate(
                        &mut state.population.chromosomes,
                        remaining_size,
                    );
                }
            }
            state.add_duration(StrategyAction::Extension, now.elapsed());
        }
    }
}

impl MassExtinction {
    pub fn new(cardinality_threshold: usize, survival_rate: f32) -> Self {
        Self {
            cardinality_threshold,
            survival_rate,
        }
    }
}
