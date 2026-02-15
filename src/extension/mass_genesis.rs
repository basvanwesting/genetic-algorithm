use super::{Extension, ExtensionEvent};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use crate::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use std::marker::PhantomData;
use std::time::Instant;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where only an Adam
/// and Eve of current best chromosomes survive. Tries to select distinct Adam and Eve when
/// genes_hash is stored on chromosome, otherwise it will just take 2 of the best (possibly
/// duplicates).
///
/// Population will recover in the following generations
#[derive(Debug, Clone)]
pub struct MassGenesis<G: EvolveGenotype> {
    _phantom: PhantomData<G>,
    pub cardinality_threshold: usize,
}

impl<G: EvolveGenotype> Extension for MassGenesis<G> {
    type Genotype = G;

    fn after_selection_complete<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
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
                        ExtensionEvent("MassGenesis".to_string()),
                        genotype,
                        state,
                        config,
                    );

                    let mut elite_chromosomes = if genotype.genes_hashing() {
                        self.extract_unique_elite_chromosomes(genotype, state, config, 2)
                    } else {
                        self.extract_elite_chromosomes(genotype, state, config, 2)
                    };
                    let elitism_size = elite_chromosomes.len();
                    let remaining_size = 2usize.saturating_sub(elitism_size);

                    state.population.truncate(remaining_size);
                    state.population.chromosomes.append(&mut elite_chromosomes);
                }
            }
            state.add_duration(StrategyAction::Extension, now.elapsed());
        }
    }
}

impl<G: EvolveGenotype> MassGenesis<G> {
    /// Create a new MassGenesis extension. Triggers when population diversity drops below threshold.
    /// Replaces all non-elite chromosomes with fresh random ones.
    /// * `cardinality_threshold` - trigger when unique chromosomes drop below this count
    pub fn new(cardinality_threshold: usize) -> Self {
        Self {
            _phantom: PhantomData,
            cardinality_threshold,
        }
    }
}
