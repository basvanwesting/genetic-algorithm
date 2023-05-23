use super::{Mutate, MutateDispatch, Mutates};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided
/// mutation_probability. Then mutates the selected chromosomes twice using random mutation. Useful
/// when a single mutation would generally not lead to improvement, because the problem space
/// behaves more like a [UniqueGenotype](crate::genotype::UniqueGenotype) where genes must be
/// swapped (but the UniqueGenotype doesn't map to the problem space well)
#[derive(Debug, Clone)]
pub struct Twice {
    pub mutation_probability: f32,
}

impl Mutate for Twice {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_random(chromosome, rng);
                genotype.mutate_chromosome_random(chromosome, rng);
            }
        }
    }
    fn report(&self) -> String {
        format!("twice: {:2.2}", self.mutation_probability)
    }
}

impl Twice {
    pub fn new(mutation_probability: f32) -> Self {
        Self {
            mutation_probability,
        }
    }
    pub fn new_dispatch(mutation_probability: f32) -> MutateDispatch {
        MutateDispatch {
            mutate: Mutates::Twice,
            mutation_probability,
            ..Default::default()
        }
    }
}
