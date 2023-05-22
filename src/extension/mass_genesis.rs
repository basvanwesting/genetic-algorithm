use super::{Extension, ExtensionDispatch, Extensions};
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::fitness::Fitness;
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::population::Population;
use crate::strategy::evolve::Evolve;
use rand::Rng;

/// A version of [MassExtinction](crate::extension::ExtensionMassExtinction), where only an adam and eve of current best chromosomes survive
#[derive(Debug, Clone)]
pub struct MassGenesis {
    pub uniformity_threshold: f32,
}

impl Extension for MassGenesis {
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
        _rng: &mut R,
    ) {
        if population.size() >= evolve.config.target_population_size
            && population.fitness_score_uniformity() >= self.uniformity_threshold
        {
            log::debug!("### mass genesis event");
            if let Some(best_chromosome) = &evolve.state.best_chromosome {
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
