use crate::chromosome::Chromosome;
use crate::gene::Gene;
use crate::population::Population;
use itertools::Itertools;
use rand::prelude::*;
use std::fmt;

pub struct Context {
    pub gene_size: usize,
    pub population_size: usize,
    pub tournament_size: usize,
    pub max_stale_generations: usize,
    pub mutation_probability: f32,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
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

    // defined here because needs to know gene type bool
    pub fn random_chromosome_factory(&self) -> Chromosome {
        //let mut genes: Vec<bool> = (0..self.gene_size).map(|_| rng.gen()).collect();
        let genes: Vec<Gene> = rand::thread_rng()
            .sample_iter(rand::distributions::Standard)
            .take(self.gene_size)
            .map(|v| Gene::new(v))
            .collect();

        Chromosome::new(genes)
    }

    pub fn permutation_population_factory(&self) -> Population {
        let chromosomes = (0..self.gene_size)
            .map(|_| [Gene::new(true), Gene::new(false)])
            .multi_cartesian_product()
            .map(|genes| Chromosome::new(genes))
            .collect();

        Population::new(chromosomes)
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            gene_size: 10,
            population_size: 100,
            tournament_size: 4,
            max_stale_generations: 20,
            mutation_probability: 0.1,
        }
    }
}

impl fmt::Display for Context {
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
