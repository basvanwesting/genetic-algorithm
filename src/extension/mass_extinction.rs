use super::{Extension, ExtensionEvent};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::time::Instant;

/// Simulates a cambrian explosion. The controlling metric is population cardinality in the
/// population after selection. When this cardinality drops to the threshold, the population is
/// randomly reduced regardless of fitness using the survival_rate (fraction of population). The
/// elitism_rate ensures the passing of the best chromosomes before random reduction starts
/// (doesn't care about best chromosome uniqueness).
///
/// Population will recover in the following generations
#[derive(Debug, Clone)]
pub struct MassExtinction {
    pub cardinality_threshold: usize,
    pub survival_rate: f32,
    pub elitism_rate: f32,
}

impl Extension for MassExtinction {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        if state.population.size() >= config.target_population_size {
            let now = Instant::now();
            if let Some(cardinality) = state.population_cardinality() {
                if cardinality <= self.cardinality_threshold {
                    reporter.on_extension_event(
                        ExtensionEvent::MassExtinction("".to_string()),
                        genotype,
                        state,
                        config,
                    );
                    let population_size = state.population.size();

                    let elitism_size = ((population_size as f32 * self.elitism_rate).ceil()
                        as usize)
                        .min(population_size);
                    let mut elite_chromosomes =
                        self.extract_elite_chromosomes(genotype, state, config, elitism_size);
                    let elitism_size = elite_chromosomes.len();

                    let remaining_size: usize = ((population_size as f32 * self.survival_rate)
                        .ceil() as usize)
                        .min(population_size)
                        .max(2);

                    let remaining_size = remaining_size.saturating_sub(elitism_size);

                    state.population.shuffle(rng);
                    state.population.chromosomes.truncate(remaining_size);

                    state.population.chromosomes.append(&mut elite_chromosomes);
                    let population_size = state.population.size();
                    // move back to front, elite_chromosomes internally unordered
                    for i in 0..elitism_size {
                        state
                            .population
                            .chromosomes
                            .swap(i, population_size - 1 - i);
                    }
                }
            }
            state.add_duration(StrategyAction::Extension, now.elapsed());
        }
    }
}

impl MassExtinction {
    pub fn new(cardinality_threshold: usize, survival_rate: f32, elitism_rate: f32) -> Self {
        Self {
            cardinality_threshold,
            survival_rate,
            elitism_rate,
        }
    }
}
