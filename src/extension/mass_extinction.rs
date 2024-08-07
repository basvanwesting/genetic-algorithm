use super::Extension;
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
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
    fn call<G: Genotype, R: Rng>(
        &mut self,
        _genotype: &G,
        evolve_config: &EvolveConfig,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        if population.size() >= evolve_config.target_population_size
            && population.fitness_score_cardinality() <= self.cardinality_threshold
        {
            log::debug!("### extension, mass extinction event");
            population.trim(self.survival_rate, rng);
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
