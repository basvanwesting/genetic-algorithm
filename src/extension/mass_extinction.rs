use super::{Extension, ExtensionDispatch, Extensions};
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use rand::Rng;

/// Simulates a cambrian explosion. The controlling metric is fitness score uniformity in the
/// population (a fraction of the population which has the same fitness score). When this
/// uniformity passes the threshold, the population is randomly reduced using the survival_rate
/// (fraction of population).
#[derive(Debug, Clone)]
pub struct MassExtinction {
    pub uniformity_threshold: f32,
    pub survival_rate: f32,
}

impl Extension for MassExtinction {
    fn call<G: Genotype, R: Rng>(
        &mut self,
        _genotype: &G,
        evolve_config: &EvolveConfig,
        _evolve_state: &EvolveState<G>,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        if population.size() >= evolve_config.target_population_size
            && population.fitness_score_uniformity() >= self.uniformity_threshold
        {
            log::debug!("### extension, mass extinction event");
            population.trim(self.survival_rate, rng);
        }
    }
}

impl MassExtinction {
    pub fn new(uniformity_threshold: f32, survival_rate: f32) -> Self {
        Self {
            uniformity_threshold,
            survival_rate,
        }
    }

    pub fn new_dispatch(uniformity_threshold: f32, survival_rate: f32) -> ExtensionDispatch {
        ExtensionDispatch {
            extension: Extensions::MassExtinction,
            uniformity_threshold,
            survival_rate,
            ..Default::default()
        }
    }
}
