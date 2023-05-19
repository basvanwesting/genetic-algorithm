use super::{Extension, ExtensionDispatch, Extensions};
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::fitness::Fitness;
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::population::Population;
use crate::strategy::evolve::Evolve;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct MassInvasion {
    pub uniformity_threshold: f32,
    pub survival_rate: f32,
}

impl Extension for MassInvasion {
    fn call<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        R: Rng,
    >(
        &self,
        evolve: &Evolve<G, M, F, S, C, E>,
        population: &mut Population<G>,
        rng: &mut R,
    ) {
        if population.fitness_score_uniformity() >= self.uniformity_threshold {
            log::debug!("### mass invasion event");
            let bool_sampler = Bernoulli::new(self.survival_rate as f64).unwrap();
            for chromosome in &mut population.chromosomes {
                if !bool_sampler.sample(rng) {
                    chromosome.genes = evolve.genotype.random_genes_factory(rng);
                    chromosome.taint_fitness_score();
                }
            }
        }
    }
}

impl MassInvasion {
    pub fn new(uniformity_threshold: f32, survival_rate: f32) -> Self {
        Self {
            uniformity_threshold,
            survival_rate,
        }
    }

    pub fn new_dispatch(uniformity_threshold: f32, survival_rate: f32) -> ExtensionDispatch {
        ExtensionDispatch {
            extension: Extensions::MassInvasion,
            uniformity_threshold,
            survival_rate,
            ..Default::default()
        }
    }
}
