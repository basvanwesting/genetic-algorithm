use crate::chromosome::Chromosome;
use crate::fitness::Fitness;
use crate::genotype::PermutableGenotype;
use crate::population::Population;
use std::fmt;

pub struct Permutate<G: PermutableGenotype, F: Fitness<Genotype = G>> {
    pub genotype: G,
    pub best_chromosome: Option<Chromosome<G>>,
    pub fitness: Option<F>,
    pub population: Population<G>,
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> Permutate<G, F> {
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

        self.population = self.genotype.population_factory();
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
}

impl<G: PermutableGenotype, F: Fitness<Genotype = G>> fmt::Display for Permutate<G, F> {
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
