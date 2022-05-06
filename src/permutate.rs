use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::PermutableGenotype;
use crate::population::Population;
use std::fmt;

pub struct Permutate<G: PermutableGenotype, F: Fitness<Genotype = G>> {
    pub genotype: G,
    pub fitness_ordering: FitnessOrdering,
    pub fitness: Option<F>,
    pub best_chromosome: Option<Chromosome<G>>,
    pub population: Population<G>,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Permutate<G, F> {
    pub fn new(genotype: G) -> Self {
        Self {
            genotype,
            fitness_ordering: FitnessOrdering::Maximize,
            fitness: None,
            best_chromosome: None,
            population: Population::new_empty(),
        }
    }

    pub fn with_fitness_ordering(mut self, fitness_ordering: FitnessOrdering) -> Self {
        self.fitness_ordering = fitness_ordering;
        self
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
        let mut fitness = self.fitness.as_ref().cloned().unwrap();

        self.population = self.genotype.population_factory();
        self.population = fitness.call_for_population(self.population);
        self.update_best_chromosome();
        self
    }

    fn update_best_chromosome(&mut self) {
        self.best_chromosome = self
            .population
            .best_chromosome(self.fitness_ordering)
            .cloned();
    }

    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> fmt::Display for Permutate<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate:")?;
        writeln!(f, "  fitness: {:?}", self.fitness.as_ref())?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  population size: {:?}", self.population.size())?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
