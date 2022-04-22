use crate::context::Context;
use crate::gene::Gene;
//use crate::global_rand;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &mut Context<T>,
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
        context: &mut Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        let gene_index_sampler = Uniform::from(0..context.gene_size);
        for chromosome in &mut population.chromosomes {
            //if global_rand::sample_bernoulli(&bool_sampler) {
            if bool_sampler.sample(rng) {
                let index = gene_index_sampler.sample(rng);
                //let index = global_rand::sample_uniform(&gene_index_sampler);
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
        context: &mut Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        for chromosome in &mut population.chromosomes {
            for gene in &mut chromosome.genes {
                //if global_rand::sample_bernoulli(&bool_sampler) {
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
        context: &mut Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        let gene_index_sampler = Uniform::from(0..context.gene_size);
        for chromosome in &mut population.chromosomes {
            //if global_rand::sample_bernoulli(&bool_sampler) {
            if bool_sampler.sample(rng) {
                let index1 = gene_index_sampler.sample(rng);
                let index2 = gene_index_sampler.sample(rng);
                //let index1 = global_rand::sample_uniform(&gene_index_sampler);
                //let index2 = global_rand::sample_uniform(&gene_index_sampler);
                chromosome.genes.swap(index1, index2);
                chromosome.taint_fitness_score();
            }
        }
        population
    }
}
