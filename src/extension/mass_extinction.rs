use super::{Extension, ExtensionEvent};
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

/// Simulates a cambrian explosion. The controlling metric is fitness score cardinality in the
/// population. When this cardinality drops to the threshold, the population is randomly reduced
/// using the survival_rate (fraction of population).
#[derive(Debug, Clone)]
pub struct MassExtinction {
    pub cardinality_threshold: usize,
    pub survival_rate: f32,
}

impl Extension for MassExtinction {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G::Allele>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        if state.population.size() >= config.target_population_size
            && state.population.fitness_score_cardinality() <= self.cardinality_threshold
        {
            reporter.on_extension_event(
                ExtensionEvent::MassExtinction("".to_string()),
                state,
                config,
            );
            state.population.trim(self.survival_rate, rng);
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
