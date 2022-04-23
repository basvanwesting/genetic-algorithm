use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &Context<T>,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T>;
}

pub type MutationProbability = f32;

#[derive(Debug, Clone)]
pub struct SingleGene(pub MutationProbability);
impl Mutate for SingleGene {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        let gene_index_sampler = Uniform::from(0..context.gene_size);
        for chromosome in &mut population.chromosomes {
            if bool_sampler.sample(rng) {
                let index = gene_index_sampler.sample(rng);
                chromosome.genes[index].mutate(context, rng);
                chromosome.taint_fitness_score();
            }
        }
        population
    }
}

#[derive(Debug, Clone)]
pub struct MultipleGene(pub MutationProbability);
impl Mutate for MultipleGene {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        for chromosome in &mut population.chromosomes {
            for gene in &mut chromosome.genes {
                if bool_sampler.sample(rng) {
                    gene.mutate(context, rng);
                }
            }
            chromosome.taint_fitness_score();
        }
        population
    }
}

#[derive(Debug, Clone)]
pub struct SwapSingleGene(pub MutationProbability);
impl Mutate for SwapSingleGene {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        let gene_index_sampler = Uniform::from(0..context.gene_size);
        for chromosome in &mut population.chromosomes {
            if bool_sampler.sample(rng) {
                let index1 = gene_index_sampler.sample(rng);
                let index2 = gene_index_sampler.sample(rng);
                chromosome.genes.swap(index1, index2);
                chromosome.taint_fitness_score();
            }
        }
        population
    }
}
