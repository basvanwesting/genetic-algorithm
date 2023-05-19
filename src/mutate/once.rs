use super::{Mutate, MutateDispatch, Mutates};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided mutation_probability. Then mutates the
/// selected chromosomes once using random mutation.
#[derive(Debug, Clone)]
pub struct Once {
    pub mutation_probability: f32,
}

impl Mutate for Once {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in &mut population.chromosomes {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_random(chromosome, rng);
            }
        }
    }
    fn report(&self) -> String {
        format!("once: {:2.2}", self.mutation_probability)
    }
}

impl Once {
    pub fn new(mutation_probability: f32) -> Self {
        Self {
            mutation_probability,
        }
    }
    pub fn new_dispatch(mutation_probability: f32) -> MutateDispatch {
        MutateDispatch {
            mutate: Mutates::Once,
            mutation_probability,
            ..Default::default()
        }
    }
}
