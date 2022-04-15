use crate::chromosome::Chromosome;
use crate::compete::Compete;
use crate::context::Context;
use crate::crossover::Crossover;
use crate::fitness::Fitness;
use crate::gene::Gene;
use crate::mutate::Mutate;
use std::fmt;

pub struct Evolve<T: Gene, M: Mutate, F: Fitness<T>, S: Crossover, C: Compete> {
    pub context: Context<T>,
    pub max_stale_generations: usize,
    pub best_chromosome: Option<Chromosome<T>>,
    pub mutate: Option<M>,
    pub fitness: Option<F>,
    pub crossover: Option<S>,
    pub compete: Option<C>,
}

impl<T: Gene, M: Mutate, F: Fitness<T>, S: Crossover, C: Compete> Evolve<T, M, F, S, C> {
    pub fn new(context: Context<T>) -> Self {
        Self {
            context: context,
            max_stale_generations: 0,
            best_chromosome: None,
            mutate: None,
            fitness: None,
            crossover: None,
            compete: None,
        }
    }

    pub fn with_max_stale_generations(mut self, max_stale_generations: usize) -> Self {
        self.max_stale_generations = max_stale_generations;
        self
    }
    pub fn with_mutate(mut self, mutate: M) -> Self {
        self.mutate = Some(mutate);
        self
    }
    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }
    pub fn with_crossover(mut self, crossover: S) -> Self {
        self.crossover = Some(crossover);
        self
    }
    pub fn with_compete(mut self, compete: C) -> Self {
        self.compete = Some(compete);
        self
    }

    pub fn call(mut self) -> Self {
        let mutate = self.mutate.as_ref().unwrap();
        let fitness = self.fitness.as_ref().unwrap();
        let crossover = self.crossover.as_ref().unwrap();
        let compete = self.compete.as_ref().unwrap();

        let mut generation = 0;
        let mut best_generation = 0;
        let mut new_population = self.context.random_population_factory();
        let mut best_chromosome = new_population.best_chromosome().unwrap().clone();

        while generation - best_generation < self.max_stale_generations {
            let mut parent_population = new_population;
            let mut child_population = crossover.call(&mut self.context, &parent_population);
            mutate.call(&mut self.context, &mut child_population);
            fitness.call_for_population(&mut child_population);
            child_population.merge(&mut parent_population);
            new_population = compete.call(&mut self.context, child_population);

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

impl<T: Gene, M: Mutate, F: Fitness<T>, S: Crossover, C: Compete> fmt::Display
    for Evolve<T, M, F, S, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "evolve:\n")?;
        write!(
            f,
            "  max_stale_generations: {}\n",
            self.max_stale_generations
        )?;
        if let Some(best_chromosome) = self.best_chromosome.as_ref() {
            write!(f, "  best chromosome: {}\n", best_chromosome)
        } else {
            write!(f, "  no best chromosome\n")
        }
    }
}
