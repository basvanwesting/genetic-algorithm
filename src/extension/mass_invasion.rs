use super::Extension;
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where the extinct population is replaced by a random population
#[derive(Debug, Clone)]
pub struct MassInvasion {
    pub cardinality_threshold: usize,
    pub survival_rate: f32,
}

impl Extension for MassInvasion {
    fn call<G: Genotype, R: Rng>(
        &mut self,
        genotype: &G,
        evolve_config: &EvolveConfig,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        if population.size() >= evolve_config.target_population_size
            && population.fitness_score_cardinality() <= self.cardinality_threshold
        {
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
}

impl MassInvasion {
    pub fn new(cardinality_threshold: usize, survival_rate: f32) -> Self {
        Self {
            cardinality_threshold,
            survival_rate,
        }
    }
}
