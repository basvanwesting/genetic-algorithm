use super::{Extension, ExtensionEvent};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use crate::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use std::time::Instant;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where only an Adam
/// and Eve of current best chromosomes survive. Tries to select distinct Adam and Eve when
/// genes_hash is stored on chromosome, otherwise it will just take 2 of the best (possibly
/// duplicates).
///
/// Population will recover in the following generations
#[derive(Debug, Clone)]
pub struct MassGenesis {
    pub cardinality_threshold: usize,
}

impl Extension for MassGenesis {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        _rng: &mut R,
    ) {
        if state.population.size() >= config.target_population_size {
            let now = Instant::now();
            if let Some(cardinality) = state.population_cardinality() {
                if cardinality <= self.cardinality_threshold {
                    reporter.on_extension_event(
                        ExtensionEvent::MassGenesis("".to_string()),
                        genotype,
                        state,
                        config,
                    );

                    let mut elite_chromosomes =
                        self.extract_unique_elite_chromosomes(genotype, state, config, 2);
                    let elitism_size = elite_chromosomes.len();
                    let remaining_size = 2usize.saturating_sub(elitism_size);

                    state.population.truncate_with_recycling(remaining_size);
                    state.population.chromosomes.append(&mut elite_chromosomes);
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
