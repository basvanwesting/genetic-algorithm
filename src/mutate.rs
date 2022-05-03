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

#[derive(Clone, Debug)]
pub enum Mutates {
    SingleGene,
}

#[derive(Clone, Debug)]
pub struct MutateDispatch(pub Mutates, pub MutationProbability);
impl Mutate for MutateDispatch {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        match self.0 {
            Mutates::SingleGene => SingleGene(self.1).call(genotype, population, rng),
        }
    }
}

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
