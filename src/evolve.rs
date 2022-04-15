use crate::chromosome::Chromosome;
use crate::competition;
use crate::context::Context;
use crate::crossover;
use crate::gene::Gene;
use crate::mutate::Mutate;

pub struct Evolve<T: Gene, M: Mutate> {
    pub context: Context<T>,
    pub best_chromosome: Option<Chromosome<T>>,
    pub mutator: M,
}

impl<T: Gene, M: Mutate> Evolve<T, M> {
    pub fn new(context: Context<T>, mutator: M) -> Self {
        Self {
            context: context,
            best_chromosome: None,
            mutator: mutator,
        }
    }

    pub fn call(mut self) -> Self {
        let mut generation = 0;
        let mut best_generation = 0;
        let mut new_population = self.context.random_population_factory();
        let mut best_chromosome = new_population.best_chromosome().unwrap().clone();

        while generation - best_generation < self.context.max_stale_generations {
            let mut parent_population = new_population;
            let mut child_population = crossover::individual(&mut self.context, &parent_population);
            self.mutator.call(&mut self.context, &mut child_population);
            child_population.calculate_fitness(&self.context);
            child_population.merge(&mut parent_population);
            new_population = competition::tournament(&mut self.context, child_population);

            generation += 1;
            println!(
                "generation {:?}, best chromosome {}",
                generation, best_chromosome
            );

            new_population.sort();
            if let Some(new_best_chromosome) = new_population.best_chromosome() {
                if new_best_chromosome > &best_chromosome {
                    best_chromosome = new_best_chromosome.clone();
                    best_generation = generation;
                }
            }
        }
        self.best_chromosome = Some(best_chromosome);
        self
    }
}
