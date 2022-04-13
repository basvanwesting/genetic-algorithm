use crate::chromosome::Chromosome;
use crate::fitness;
use crate::gene::{Gene, GeneTrait};
use crate::population::Population;
use itertools::Itertools;
use rand::prelude::*;
use rand::seq::SliceRandom;
//use rand::seq::IteratorRandom;
use std::fmt;

pub struct Context<T: GeneTrait> {
    pub gene_size: usize,
    pub gene_values: Vec<Gene<T>>,
    pub population_size: usize,
    pub tournament_size: usize,
    pub max_stale_generations: usize,
    pub mutation_probability: f32,
    pub fitness_function: fn(&Chromosome<T>) -> usize,
}

impl<T: GeneTrait> Context<T> {
    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_gene_values(mut self, gene_values: Vec<Gene<T>>) -> Self {
        self.gene_values = gene_values;
        self
    }

    pub fn with_population_size(mut self, population_size: usize) -> Self {
        self.population_size = population_size;
        self
    }

    pub fn with_tournament_size(mut self, tournament_size: usize) -> Self {
        self.tournament_size = tournament_size;
        self
    }

    pub fn with_max_stale_generations(mut self, max_stale_generations: usize) -> Self {
        self.max_stale_generations = max_stale_generations;
        self
    }

    pub fn with_mutation_probability(mut self, mutation_probability: f32) -> Self {
        self.mutation_probability = mutation_probability;
        self
    }

    pub fn with_fitness_function(mut self, fitness_function: fn(&Chromosome<T>) -> usize) -> Self {
        self.fitness_function = fitness_function;
        self
    }

    pub fn permutation_population_factory(&self) -> Population<T> {
        let chromosomes = (0..self.gene_size)
            .map(|_| self.gene_values.clone())
            .multi_cartesian_product()
            .map(|genes| Chromosome::new(genes))
            .collect();

        Population::new(chromosomes)
    }

    pub fn random_chromosome_factory(&self) -> Chromosome<T> {
        let mut rng = thread_rng();
        let genes: Vec<Gene<T>> = (0..self.gene_size)
            .map(|_| self.gene_values.choose(&mut rng).unwrap().clone())
            .collect();
        Chromosome::new(genes)
    }

    pub fn random_population_factory(&self) -> Population<T> {
        let chromosomes = (0..self.population_size)
            .map(|_| self.random_chromosome_factory())
            .collect();
        Population::new(chromosomes)
    }
}

impl Context<bool> {
    pub fn new() -> Self {
        Self::default()
    }

    //pub fn random_chromosome_factory(&self) -> Chromosome<bool> {
    ////let mut genes: Vec<bool> = (0..self.gene_size).map(|_| rng.gen()).collect();
    //let genes: Vec<Gene<bool>> = rand::thread_rng()
    //.sample_iter(rand::distributions::Standard)
    //.take(self.gene_size)
    //.map(|v| Gene(v))
    //.collect();
    //Chromosome::new(genes)
    //}

    //pub fn random_population_factory(&self) -> Population<bool> {
    //let chromosomes = (0..self.population_size)
    //.map(|_| self.random_chromosome_factory())
    //.collect();
    //Population::new(chromosomes)
    //}
}

impl Context<u8> {
    pub fn new() -> Self {
        Self::default()
    }

    //pub fn random_chromosome_factory(&self) -> Chromosome<u8> {
    //let genes: Vec<Gene<u8>> = rand::thread_rng()
    //.sample_iter(rand::distributions::Standard)
    //.take(self.gene_size)
    //.map(|v| Gene(v))
    //.collect();
    //Chromosome::new(genes)
    //}

    //pub fn random_population_factory(&self) -> Population<u8> {
    //let chromosomes = (0..self.population_size)
    //.map(|_| self.random_chromosome_factory())
    //.collect();
    //Population::new(chromosomes)
    //}
}

impl Default for Context<bool> {
    fn default() -> Self {
        Context {
            gene_size: 10,
            gene_values: vec![Gene(true), Gene(false)],
            population_size: 100,
            tournament_size: 4,
            max_stale_generations: 20,
            mutation_probability: 0.1,
            fitness_function: fitness::count_true_values,
        }
    }
}

impl Default for Context<u8> {
    fn default() -> Self {
        Context {
            gene_size: 10,
            gene_values: vec![Gene(1), Gene(2), Gene(3), Gene(4)],
            population_size: 100,
            tournament_size: 4,
            max_stale_generations: 20,
            mutation_probability: 0.1,
            fitness_function: fitness::sum_values,
        }
    }
}

impl<T: GeneTrait> fmt::Display for Context<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "context:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  population_size: {}\n", self.population_size)?;
        write!(f, "  tournament_size: {}\n", self.tournament_size)?;
        write!(
            f,
            "  max_stale_generations: {}\n",
            self.max_stale_generations
        )?;
        write!(f, "  mutation_probability: {}\n", self.mutation_probability)
    }
}
