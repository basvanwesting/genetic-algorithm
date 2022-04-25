use crate::chromosome::Chromosome;
use crate::fitness::Fitness;
use crate::gene::Gene;
use crate::genotype::Genotype;
use crate::population::Population;
use itertools::Itertools;
use std::fmt;

pub trait PermutableGenotype<T: Gene>: Genotype<T> {
    fn gene_values_to_permutate(&self) -> Vec<T>;
}

pub struct Permutate<T: Gene, G: PermutableGenotype<T>, F: Fitness<T>> {
    pub genotype: G,
    pub best_chromosome: Option<Chromosome<T>>,
    pub fitness: Option<F>,
    pub population: Population<T>,
}

impl<T: Gene, G: PermutableGenotype<T>, F: Fitness<T>> Permutate<T, G, F> {
    pub fn new(genotype: G) -> Self {
        Self {
            genotype: genotype,
            fitness: None,
            best_chromosome: None,
            population: Population::new_empty(),
        }
    }

    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }

    pub fn is_valid(&self) -> bool {
        self.fitness.is_some()
    }

    pub fn call(self) -> Self {
        if !self.is_valid() {
            return self;
        }
        self.execute()
    }

    fn execute(mut self) -> Self {
        let fitness = self.fitness.as_ref().cloned().unwrap();

        self.population = self.population_factory();
        self.population = fitness.call_for_population(self.population);
        self.update_best_chromosome();
        self
    }

    fn update_best_chromosome(&mut self) {
        if self.best_chromosome.as_ref() < self.population.best_chromosome() {
            self.best_chromosome = self.population.best_chromosome().cloned();
        }
    }

    fn best_fitness_score(&self) -> Option<isize> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }

    pub fn population_factory(&self) -> Population<T> {
        let chromosomes = (0..self.genotype.gene_size())
            .map(|_| self.genotype.gene_values_to_permutate())
            .multi_cartesian_product()
            .map(|genes| Chromosome::new(genes))
            .collect();

        Population::new(chromosomes)
    }
}

impl<T: Gene, G: PermutableGenotype<T>, F: Fitness<T>> fmt::Display for Permutate<T, G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "permutate:\n")?;
        write!(f, "  fitness: {:?}\n", self.fitness.as_ref())?;
        write!(f, "  population size: {:?}\n", self.population.size())?;
        write!(f, "  best fitness score: {:?}\n", self.best_fitness_score())?;
        write!(
            f,
            "  best_chromosome: {:?}\n",
            self.best_chromosome.as_ref()
        )
    }
}
