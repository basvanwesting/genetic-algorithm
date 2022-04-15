use crate::chromosome::Chromosome;
use crate::compete::Compete;
use crate::context::Context;
use crate::crossover::Crossover;
use crate::fitness::Fitness;
use crate::gene::Gene;
use crate::mutate::Mutate;

pub struct Evolve<T: Gene, M: Mutate, F: Fitness<T>, S: Crossover, C: Compete> {
    pub context: Context<T>,
    pub best_chromosome: Option<Chromosome<T>>,
    pub mutator: M,
    pub fitness: F,
    pub crossover: S,
    pub compete: C,
}

impl<T: Gene, M: Mutate, F: Fitness<T>, S: Crossover, C: Compete> Evolve<T, M, F, S, C> {
    pub fn new(context: Context<T>, mutator: M, fitness: F, crossover: S, compete: C) -> Self {
        Self {
            context: context,
            best_chromosome: None,
            mutator: mutator,
            fitness: fitness,
            crossover: crossover,
            compete: compete,
        }
    }

    pub fn call(mut self) -> Self {
        let mut generation = 0;
        let mut best_generation = 0;
        let mut new_population = self.context.random_population_factory();
        let mut best_chromosome = new_population.best_chromosome().unwrap().clone();

        while generation - best_generation < self.context.max_stale_generations {
            let mut parent_population = new_population;
            let mut child_population = self.crossover.call(&mut self.context, &parent_population);
            self.mutator.call(&mut self.context, &mut child_population);
            self.fitness.call_for_population(&mut child_population);
            child_population.merge(&mut parent_population);
            new_population = self.compete.call(&mut self.context, child_population);

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
