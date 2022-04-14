use crate::chromosome::Chromosome;
use crate::competition;
use crate::context::Context;
use crate::crossover;
use crate::gene::Gene;
use crate::mutation;

pub struct Evolve<T: Gene> {
    pub context: Context<T>,
    pub best_chromosome: Option<Chromosome<T>>,
}

impl<T: Gene> Evolve<T> {
    pub fn new(context: Context<T>) -> Self {
        Self {
            context: context,
            best_chromosome: None,
        }
    }

    pub fn call(&mut self) {
        let mut generation = 0;
        let mut best_generation = 0;
        let mut new_population = self.context.random_population_factory();
        let mut best_chromosome = new_population.best_chromosome().unwrap().clone();

        while generation - best_generation < self.context.max_stale_generations {
            let mut parent_population = new_population;
            let mut child_population = crossover::individual(&mut self.context, &parent_population);
            mutation::single_gene(&mut self.context, &mut child_population);
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
    }
}
