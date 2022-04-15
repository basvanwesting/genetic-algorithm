use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::fitness::Fitness;
use crate::gene::Gene;

pub struct Permutate<T: Gene, F: Fitness<T>> {
    pub context: Context<T>,
    pub best_chromosome: Option<Chromosome<T>>,
    pub fitness: F,
}

impl<T: Gene, F: Fitness<T>> Permutate<T, F> {
    pub fn new(context: Context<T>, fitness: F) -> Self {
        Self {
            context: context,
            best_chromosome: None,
            fitness: fitness,
        }
    }

    pub fn call(mut self) -> Self {
        let mut population = self.context.permutation_population_factory();
        self.fitness.call_for_population(&mut population);
        population.sort();
        if let Some(best_chromosome) = population.best_chromosome() {
            self.best_chromosome = Some(best_chromosome.clone());
        }
        self
    }
}
