use super::Mutate;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population](crate::population::Population) with the provided mutation_probability. Then mutates the
/// selected chromosomes once using random mutation.
#[derive(Debug, Clone)]
pub struct SingleGeneRandom {
    pub mutation_probability: f32,
}

impl Mutate for SingleGeneRandom {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in state
            .population
            .chromosomes
            .iter_mut()
            .filter(|c| c.age == 0)
        {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_random(chromosome, rng);
            }
        }
    }
    fn report(&self) -> String {
        format!("single-gene-random: {:2.2}", self.mutation_probability)
    }
}

impl SingleGeneRandom {
    pub fn new(mutation_probability: f32) -> Self {
        Self {
            mutation_probability,
        }
    }
}
