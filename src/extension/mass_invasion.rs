use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where the extinct population is replaced by a random population
#[derive(Debug, Clone)]
pub struct MassInvasion {
    pub uniformity_threshold: f32,
    pub survival_rate: f32,
}

impl MassInvasion {
    pub fn call<G: Genotype, R: Rng>(
        &mut self,
        genotype: &G,
        _evolve_config: &EvolveConfig,
        _evolve_state: &EvolveState<G>,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        if population.fitness_score_uniformity() >= self.uniformity_threshold {
            log::debug!("### mass invasion event");
            let bool_sampler = Bernoulli::new(self.survival_rate as f64).unwrap();
            for chromosome in &mut population.chromosomes {
                if !bool_sampler.sample(rng) {
                    chromosome.genes = genotype.random_genes_factory(rng);
                    chromosome.taint_fitness_score();
                }
            }
        }
    }
    pub fn new(uniformity_threshold: f32, survival_rate: f32) -> Self {
        Self {
            uniformity_threshold,
            survival_rate,
        }
    }
}
