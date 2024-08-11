use super::{Extension, ExtensionEvent};
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where only an adam and eve of current best chromosomes survive
#[derive(Debug, Clone)]
pub struct MassGenesis {
    pub cardinality_threshold: usize,
}

impl Extension for MassGenesis {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G::Allele>,
        config: &EvolveConfig,
        reporter: &mut SR,
        _rng: &mut R,
    ) {
        if state.population.size() >= config.target_population_size
            && state.population.fitness_score_cardinality() <= self.cardinality_threshold
        {
            reporter.on_extension_event(ExtensionEvent::MassGenesis("".to_string()), state, config);
            if let Some(best_chromosome) = state.population.best_chromosome(config.fitness_ordering)
            {
                state.population.chromosomes =
                    vec![best_chromosome.clone(), best_chromosome.clone()]
            }
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
