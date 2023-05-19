use super::{Extension, ExtensionDispatch, Extensions};
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::fitness::Fitness;
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::population::Population;
use crate::strategy::evolve::Evolve;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct MassExtinction {
    pub uniformity_threshold: f32,
    pub survival_rate: f32,
}

impl Extension for MassExtinction {
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
        if population.size() >= evolve.population_size
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
