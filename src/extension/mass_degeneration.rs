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

/// Simulates a cambrian explosion. The controlling metric is fitness score uniformity in the
/// population (a fraction of the population which has the same fitness score). When this
/// uniformity passes the threshold, the population is randomly mutated N number of rounds.
#[derive(Debug, Clone)]
pub struct MassDegeneration {
    pub uniformity_threshold: f32,
    pub number_of_rounds: usize,
}

impl Extension for MassDegeneration {
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
            log::debug!("### mass degeneration event");

            let bool_sampler = Bernoulli::new(0.2 as f64).unwrap();
            for _ in 0..self.number_of_rounds {
                for chromosome in &mut population.chromosomes {
                    if bool_sampler.sample(rng) {
                        evolve.genotype.mutate_chromosome_random(chromosome, rng);
                    }
                }
            }
        }
    }
}

impl MassDegeneration {
    pub fn new(uniformity_threshold: f32, number_of_rounds: usize) -> Self {
        Self {
            uniformity_threshold,
            number_of_rounds,
        }
    }

    pub fn new_dispatch(uniformity_threshold: f32, number_of_rounds: usize) -> ExtensionDispatch {
        ExtensionDispatch {
            extension: Extensions::MassDegeneration,
            uniformity_threshold,
            number_of_rounds,
            ..Default::default()
        }
    }
}
