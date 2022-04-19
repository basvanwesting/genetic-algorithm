use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution, Uniform};

pub trait Mutate: Clone + std::fmt::Debug {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: Population<T>) -> Population<T>;
}

pub type MutationProbability = f32;

#[derive(Debug, Clone)]
pub struct SingleGene(pub MutationProbability);
impl Mutate for SingleGene {
    fn call<T: Gene>(
        &self,
        context: &mut Context<T>,
        mut population: Population<T>,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        let gene_index_sampler = Uniform::from(0..context.gene_size);
        for chromosome in &mut population.chromosomes {
            if bool_sampler.sample(&mut context.rng) {
                let index = gene_index_sampler.sample(&mut context.rng);
                chromosome.genes[index].mutate(context);
                chromosome.taint_fitness_score();
            }
        }
        population
    }
}

#[derive(Debug, Clone)]
pub struct MultipleGene(pub MutationProbability);
impl Mutate for MultipleGene {
    fn call<T: Gene>(
        &self,
        context: &mut Context<T>,
        mut population: Population<T>,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(self.0 as f64).unwrap();
        for chromosome in &mut population.chromosomes {
            for gene in &mut chromosome.genes {
                if bool_sampler.sample(&mut context.rng) {
                    gene.mutate(context);
                }
            }
            chromosome.taint_fitness_score();
        }
        population
    }
}
