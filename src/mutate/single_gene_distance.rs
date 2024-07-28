use super::Mutate;
use crate::genotype::{ContinuousGenotypeAllele, Genotype};
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;
use std::ops::Range;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided
/// mutation_probability. Then mutates the selected chromosomes, changing a single gene by adding
/// or substracting a uniform sample from the provided allele_distance_range.
/// Note: Only Implemented for [ContinuousGenotype](crate::genotype::ContinuousGenotype)
#[derive(Debug, Clone)]
pub struct SingleGeneDistance {
    pub mutation_probability: f32,
    pub allele_distance_range: Range<ContinuousGenotypeAllele>,
}

impl Mutate for SingleGeneDistance {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        let allele_distance_sampler = Uniform::from(self.allele_distance_range.clone());
        let sign_sampler = Bernoulli::new(0.5).unwrap();
        for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
            if bool_sampler.sample(rng) {
                let distance = allele_distance_sampler.sample(rng);
                if sign_sampler.sample(rng) {
                    genotype.mutate_chromosome_distance(chromosome, distance, rng);
                } else {
                    genotype.mutate_chromosome_distance(chromosome, -distance, rng);
                }
            }
        }
    }
    fn report(&self) -> String {
        format!(
            "single-gene-distance: {:2.2}, {:?}",
            self.mutation_probability, self.allele_distance_range
        )
    }
}

impl SingleGeneDistance {
    pub fn new(
        mutation_probability: f32,
        allele_distance_range: Range<ContinuousGenotypeAllele>,
    ) -> Self {
        Self {
            mutation_probability,
            allele_distance_range,
        }
    }
}
