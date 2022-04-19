use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};

pub trait Crossover: Clone + std::fmt::Debug {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: Population<T>) -> Population<T>;
}

pub type KeepParent = bool;

#[derive(Clone, Debug)]
pub struct Individual(pub KeepParent);
impl Crossover for Individual {
    fn call<T: Gene>(
        &self,
        context: &mut Context<T>,
        mut population: Population<T>,
    ) -> Population<T> {
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

                    // no need to taint_fitness_score as it is initialized with None
                    child_chromosomes.push(Chromosome::new(child_father_genes));
                    child_chromosomes.push(Chromosome::new(child_mother_genes));
                }
                _ => {}
            }
        }

        if self.0 {
            child_chromosomes.append(&mut population.chromosomes);
        }
        Population::new(child_chromosomes)
    }
}
