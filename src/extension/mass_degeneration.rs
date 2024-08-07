use super::{Extension, ExtensionEvent};
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Simulates a cambrian explosion. The controlling metric is fitness score cardinality in the
/// population. When this cardinality drops to the threshold, the population is randomly mutated N
/// number of rounds.
#[derive(Debug, Clone)]
pub struct MassDegeneration {
    pub cardinality_threshold: usize,
    pub number_of_rounds: usize,
}

impl Extension for MassDegeneration {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        evolve_config: &EvolveConfig,
        population: &mut Population<G>,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        if population.size() >= evolve_config.target_population_size
            && population.fitness_score_cardinality() <= self.cardinality_threshold
        {
            reporter.on_extension_event(ExtensionEvent::MassDegeneration("".to_string()));
            let bool_sampler = Bernoulli::new(0.2).unwrap();
            for _ in 0..self.number_of_rounds {
                for chromosome in &mut population.chromosomes {
                    if bool_sampler.sample(rng) {
                        genotype.mutate_chromosome_random(chromosome, rng);
                    }
                }
            }
        }
    }
}

impl MassDegeneration {
    pub fn new(cardinality_threshold: usize, number_of_rounds: usize) -> Self {
        Self {
            cardinality_threshold,
            number_of_rounds,
        }
    }
}
