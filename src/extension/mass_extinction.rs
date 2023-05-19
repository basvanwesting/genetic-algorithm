use super::{Extension, ExtensionDispatch, Extensions};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct MassExtinction {
    pub uniformity_threshold: f32,
    pub survival_rate: f32,
    pub minimal_population_size: usize,
}

impl Extension for MassExtinction {
    fn call<T: Genotype, R: Rng>(
        &self,
        _genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        if population.size() >= self.minimal_population_size
            && population.fitness_score_uniformity() >= self.uniformity_threshold
        {
            log::debug!("### extension, mass extinction event");
            population.trim(self.survival_rate, rng);
        }
    }
}

impl MassExtinction {
    pub fn new(
        uniformity_threshold: f32,
        survival_rate: f32,
        minimal_population_size: usize,
    ) -> Self {
        Self {
            uniformity_threshold,
            survival_rate,
            minimal_population_size,
        }
    }

    pub fn new_dispatch(
        uniformity_threshold: f32,
        survival_rate: f32,
        minimal_population_size: usize,
    ) -> ExtensionDispatch {
        ExtensionDispatch {
            extension: Extensions::MassExtinction,
            uniformity_threshold,
            survival_rate,
            minimal_population_size,
            ..Default::default()
        }
    }
}
