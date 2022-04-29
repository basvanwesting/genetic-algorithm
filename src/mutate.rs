use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T>;
}

pub type MutationProbability = f32;

#[derive(Debug, Clone)]
pub struct SingleGene(pub MutationProbability);
impl Mutate for SingleGene {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        //let gene_index_sampler = Uniform::from(0..genotype.gene_size());
        for chromosome in &mut population.chromosomes {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome(chromosome, rng);
            }
        }
        population
    }
}
