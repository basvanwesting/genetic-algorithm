use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;

pub trait Crossover: Clone + std::fmt::Debug {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &Context<T>,
        population: Population<T>,
        rng: &mut R,
    ) -> Population<T>;
}

pub type KeepParent = bool;

#[derive(Clone, Debug)]
pub struct Individual(pub KeepParent);
impl Crossover for Individual {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let gene_index_sampler = Uniform::from(0..context.gene_size);
        let mut child_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(context.population_size);

        for chunk in population.chromosomes.chunks(2) {
            match &chunk[..] {
                [father, mother] => {
                    let index = gene_index_sampler.sample(rng);
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

#[derive(Clone, Debug)]
pub struct All(pub KeepParent);
impl Crossover for All {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let bool_sampler = Bernoulli::new(0.5).unwrap();
        let mut child_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(context.population_size);

        for chunk in population.chromosomes.chunks(2) {
            match &chunk[..] {
                [father, mother] => {
                    let mut child_father_genes = father.genes.clone();
                    let mut child_mother_genes = mother.genes.clone();

                    for index in 0..(context.gene_size) {
                        if bool_sampler.sample(rng) {
                            child_father_genes[index] = mother.genes[index];
                            child_mother_genes[index] = father.genes[index];
                        }
                    }

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

#[derive(Clone, Debug)]
pub struct Range(pub KeepParent);
impl Crossover for Range {
    fn call<T: Gene, R: Rng>(
        &self,
        context: &Context<T>,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        let gene_index_sampler = Uniform::from(0..context.gene_size);
        let mut child_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(context.population_size);

        for chunk in population.chromosomes.chunks(2) {
            match &chunk[..] {
                [father, mother] => {
                    let index = gene_index_sampler.sample(rng);
                    let mut child_father_genes = father.genes.clone();
                    let mut child_mother_genes = mother.genes.clone();

                    let mut child_father_genes_split = child_father_genes.split_off(index);
                    let mut child_mother_genes_split = child_mother_genes.split_off(index);

                    child_father_genes.append(&mut child_mother_genes_split);
                    child_mother_genes.append(&mut child_father_genes_split);

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

#[derive(Clone, Debug)]
pub struct Cloning;
impl Crossover for Cloning {
    fn call<T: Gene, R: Rng>(
        &self,
        _context: &Context<T>,
        mut population: Population<T>,
        _rng: &mut R,
    ) -> Population<T> {
        let mut clones = population.clone();
        population.merge(&mut clones);
        population
    }
}
