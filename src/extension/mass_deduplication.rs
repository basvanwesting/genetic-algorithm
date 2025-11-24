use super::{Extension, ExtensionEvent};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::Rng;
use std::marker::PhantomData;
use std::time::Instant;

/// Simulates a cambrian explosion. The controlling metric is population cardinality in the
/// population after selection. When this cardinality drops to the threshold, the population is
/// reduced to only the unique individuals. Only works when genes_hash is stored on chromosome, as
/// this is the uniqueness key, otherwise the extension is ignored.
///
/// Population will recover in the following generations
#[derive(Debug, Clone)]
pub struct MassDeduplication<G: EvolveGenotype> {
    _phantom: PhantomData<G>,
    pub cardinality_threshold: usize,
}

impl<G: EvolveGenotype> Extension for MassDeduplication<G> {
    type Genotype = G;

    fn after_selection_complete<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        _rng: &mut R,
    ) {
        if genotype.genes_hashing() && state.population.size() >= config.target_population_size {
            let now = Instant::now();
            if let Some(cardinality) = state.population_cardinality() {
                if cardinality <= self.cardinality_threshold {
                    reporter.on_extension_event(
                        ExtensionEvent("MassDeduplication".to_string()),
                        genotype,
                        state,
                        config,
                    );

                    let mut unique_chromosomes =
                        self.extract_unique_chromosomes(genotype, state, config);
                    let unique_size = unique_chromosomes.len();

                    let remaining_size = 2usize.saturating_sub(unique_size);
                    state.population.truncate(remaining_size);
                    state.population.chromosomes.append(&mut unique_chromosomes);
                }
            }
            state.add_duration(StrategyAction::Extension, now.elapsed());
        }
    }
}

impl<G: EvolveGenotype> MassDeduplication<G> {
    pub fn new(cardinality_threshold: usize) -> Self {
        Self {
            _phantom: PhantomData,
            cardinality_threshold,
        }
    }
}
