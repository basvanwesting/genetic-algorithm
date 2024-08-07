use super::Mutate;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided
/// mutation_probability. Then mutates the selected chromosomes the provided number of times using
/// random mutation. Useful when a single mutation would generally not lead to improvement, because
/// the problem space behaves more like a [UniqueGenotype](crate::genotype::UniqueGenotype) where
/// genes must be swapped (but the UniqueGenotype doesn't map to the problem space well). Set
/// number_of_mutations to two in that situation.
#[derive(Debug, Clone)]
pub struct MultiGeneRandom {
    pub mutation_probability: f32,
    pub number_of_mutations: usize,
}

impl Mutate for MultiGeneRandom {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
            if bool_sampler.sample(rng) {
                for _ in 0..self.number_of_mutations {
                    genotype.mutate_chromosome_random(chromosome, rng);
                }
            }
        }
    }
    fn report(&self) -> String {
        format!(
            "multi_gene_random: {:2.2}, {}",
            self.mutation_probability, self.number_of_mutations
        )
    }
}

impl MultiGeneRandom {
    pub fn new(mutation_probability: f32, number_of_mutations: usize) -> Self {
        Self {
            mutation_probability,
            number_of_mutations,
        }
    }
}
