use crate::chromosome::Chromosome;
use crate::fitness;
use crate::gene::{BinaryGene, DiscreteGene, Gene};
use crate::population::Population;
use itertools::Itertools;
use rand::prelude::*;
use rand::seq::SliceRandom;
//use rand::seq::IteratorRandom;
use std::fmt;

pub struct Context<T: Gene> {
    pub gene_size: usize,
    pub gene_values: Vec<T>,
    pub population_size: usize,
    pub tournament_size: usize,
    pub max_stale_generations: usize,
    pub mutation_probability: f32,
    pub fitness_function: fn(&Chromosome<T>) -> usize,
}

impl<T: Gene> Context<T> {
    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_gene_values(mut self, gene_values: Vec<T>) -> Self {
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
        let genes: Vec<T> = (0..self.gene_size)
            .map(|_| *self.gene_values.choose(&mut rng).unwrap())
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

impl Context<BinaryGene> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Context<DiscreteGene> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Context<BinaryGene> {
    fn default() -> Self {
        Context {
            gene_size: 10,
            gene_values: vec![true, false],
            population_size: 100,
            tournament_size: 4,
            max_stale_generations: 20,
            mutation_probability: 0.1,
            fitness_function: fitness::count_true_values,
        }
    }
}

impl Default for Context<DiscreteGene> {
    fn default() -> Self {
        Context {
            gene_size: 10,
            gene_values: vec![1, 2, 3, 4],
            population_size: 100,
            tournament_size: 4,
            max_stale_generations: 20,
            mutation_probability: 0.1,
            fitness_function: fitness::sum_values,
        }
    }
}

impl<T: Gene> fmt::Display for Context<T> {
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
