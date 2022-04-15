use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};

pub trait Crossover: Clone + std::fmt::Debug {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: &Population<T>) -> Population<T>;
}

#[derive(Clone, Debug)]
pub struct Individual;
impl Crossover for Individual {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: &Population<T>) -> Population<T> {
        let gene_range = Uniform::from(0..context.gene_size);

        let mut child_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(context.population_size);

        for chunk in population.chromosomes.chunks(2) {
            match &chunk[..] {
                [father, mother] => {
                    let index = gene_range.sample(&mut context.rng);
                    let mut child_father_genes = father.genes.clone();
                    let mut child_mother_genes = mother.genes.clone();

                    child_father_genes[index] = mother.genes[index];
                    child_mother_genes[index] = father.genes[index];

                    child_chromosomes.push(Chromosome::new(child_father_genes));
                    child_chromosomes.push(Chromosome::new(child_mother_genes));
                }
                _ => {}
            }
        }

        Population::new(child_chromosomes)
    }
}
