use super::{Extension, ExtensionDispatch, Extensions};
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use rand::Rng;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where only an adam and eve of current best chromosomes survive
#[derive(Debug, Clone)]
pub struct MassGenesis {
    pub uniformity_threshold: f32,
}

impl Extension for MassGenesis {
    fn call<G: Genotype, R: Rng>(
        &mut self,
        _genotype: &G,
        evolve_config: &EvolveConfig,
        evolve_state: &EvolveState<G>,
        population: &mut Population<G>,
        _rng: &mut R,
    ) {
        if population.size() >= evolve_config.target_population_size
            && population.fitness_score_uniformity() >= self.uniformity_threshold
        {
            log::debug!("### mass genesis event");
            if let Some(best_chromosome) = &evolve_state.best_chromosome {
                population.chromosomes = vec![best_chromosome.clone(), best_chromosome.clone()]
            }
        }
    }
}

impl MassGenesis {
    pub fn new(uniformity_threshold: f32) -> Self {
        Self {
            uniformity_threshold,
        }
    }

    pub fn new_dispatch(uniformity_threshold: f32) -> ExtensionDispatch {
        ExtensionDispatch {
            extension: Extensions::MassGenesis,
            uniformity_threshold,
            ..Default::default()
        }
    }
}
