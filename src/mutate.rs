use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

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
        let gene_range = Uniform::from(0..context.gene_size);
        for chromosome in &mut population.chromosomes {
            if context.rng.gen::<f32>() <= self.0 {
                let index = gene_range.sample(&mut context.rng);
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
        for chromosome in &mut population.chromosomes {
            for gene in &mut chromosome.genes {
                if context.rng.gen::<f32>() <= self.0 {
                    gene.mutate(context);
                }
            }
            chromosome.taint_fitness_score();
        }
        population
    }
}
