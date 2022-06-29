use super::{Mutate, RandomMutationProbability};
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population] with the provided [RandomMutationProbability]. Then mutates the
/// selected chromosomes once using random mutation.
#[derive(Debug, Clone)]
pub struct Random(pub RandomMutationProbability);
impl Mutate for Random {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R) {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        for chromosome in &mut population.chromosomes {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_random(chromosome, rng);
            }
        }
    }
}
