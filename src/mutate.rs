use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub trait Mutate: std::fmt::Debug {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: &mut Population<T>);
}

pub type MutationProbability = f32;

#[derive(Debug)]
pub struct SingleGene(pub MutationProbability);
impl Mutate for SingleGene {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: &mut Population<T>) {
        let gene_range = Uniform::from(0..context.gene_size);
        for chromosome in &mut population.chromosomes {
            if context.rng.gen::<f32>() <= self.0 {
                let index = gene_range.sample(&mut context.rng);
                chromosome.genes[index].mutate(context);
            }
        }
    }
}

#[derive(Debug)]
pub struct MultipleGene(pub MutationProbability);
impl Mutate for MultipleGene {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: &mut Population<T>) {
        for chromosome in &mut population.chromosomes {
            for gene in &mut chromosome.genes {
                if context.rng.gen::<f32>() <= self.0 {
                    gene.mutate(context);
                }
            }
        }
    }
}
