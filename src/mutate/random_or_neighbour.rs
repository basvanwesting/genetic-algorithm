use super::{Mutate, NeighbourMutationProbability, RandomMutationProbability};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided [RandomMutationProbability] and [NeighbourMutationProbability]. Then mutates the
/// selected chromosomes once using random mutation or neighbouring mutation
#[derive(Debug, Clone)]
pub struct RandomOrNeighbour(
    pub RandomMutationProbability,
    pub NeighbourMutationProbability,
);
impl Mutate for RandomOrNeighbour {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R) {
        let sampler = Uniform::from(0.0..1.0);
        let random_threshold = self.0;
        let neighbour_threshold = self.0 + self.1;
        for chromosome in &mut population.chromosomes {
            match sampler.sample(rng) {
                p if p < random_threshold => genotype.mutate_chromosome_random(chromosome, rng),
                p if p < neighbour_threshold => {
                    genotype.mutate_chromosome_neighbour(chromosome, rng)
                }
                _ => (),
            }
        }
    }
}
