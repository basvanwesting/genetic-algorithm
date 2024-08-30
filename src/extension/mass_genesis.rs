use super::{Extension, ExtensionEvent};
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use crate::strategy::StrategyState;
use rand::Rng;
use std::time::Instant;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where only an adam and eve of current best chromosomes survive
#[derive(Debug, Clone)]
pub struct MassGenesis {
    pub cardinality_threshold: usize,
}

impl Extension for MassGenesis {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G::Allele>,
        config: &EvolveConfig,
        reporter: &mut SR,
        _rng: &mut R,
    ) {
        let now = Instant::now();
        if state.population.size() >= config.target_population_size
            && state.population.fitness_score_cardinality() <= self.cardinality_threshold
        {
            reporter.on_extension_event(ExtensionEvent::MassGenesis("".to_string()), state, config);
            if let Some(best_chromosome) = state
                .population
                .best_chromosome(config.fitness_ordering)
                .cloned()
            {
                state.population.truncate(0);
                state.population.chromosomes.push(best_chromosome.clone());
                state.population.chromosomes.push(best_chromosome);
            }
        }
        state.add_duration("extension", now.elapsed());
    }
}

impl MassGenesis {
    pub fn new(cardinality_threshold: usize) -> Self {
        Self {
            cardinality_threshold,
        }
    }
}
