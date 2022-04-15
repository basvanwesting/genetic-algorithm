use crate::chromosome::Chromosome;
use crate::compete::Compete;
use crate::context::Context;
use crate::crossover::Crossover;
use crate::fitness::Fitness;
use crate::gene::Gene;
use crate::mutate::Mutate;
use crate::population::Population;
use std::fmt;

pub struct Evolve<T: Gene, M: Mutate, F: Fitness<T>, S: Crossover, C: Compete> {
    pub context: Context<T>,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<usize>,
    pub mutate: Option<M>,
    pub fitness: Option<F>,
    pub crossover: Option<S>,
    pub compete: Option<C>,
    pub current_generation: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<T>>,
}

impl<T: Gene, M: Mutate, F: Fitness<T>, S: Crossover, C: Compete> Evolve<T, M, F, S, C> {
    pub fn new(context: Context<T>) -> Self {
        Self {
            context: context,
            max_stale_generations: None,
            target_fitness_score: None,
            mutate: None,
            fitness: None,
            crossover: None,
            compete: None,
            current_generation: 0,
            best_generation: 0,
            best_chromosome: None,
        }
    }

    pub fn with_max_stale_generations(mut self, max_stale_generations: usize) -> Self {
        self.max_stale_generations = Some(max_stale_generations);
        self
    }

    pub fn with_target_fitness_score(mut self, target_fitness_score: usize) -> Self {
        self.target_fitness_score = Some(target_fitness_score);
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

    pub fn is_valid(&self) -> bool {
        (self.max_stale_generations.is_some() || self.target_fitness_score.is_some())
            && self.mutate.is_some()
            && self.fitness.is_some()
            && self.crossover.is_some()
            && self.compete.is_some()
    }

    pub fn call(self) -> Self {
        if !self.is_valid() {
            return self;
        }
        self.execute()
    }

    fn execute(mut self) -> Self {
        let mutate = self.mutate.as_ref().cloned().unwrap();
        let fitness = self.fitness.as_ref().cloned().unwrap();
        let crossover = self.crossover.as_ref().cloned().unwrap();
        let compete = self.compete.as_ref().cloned().unwrap();

        self.current_generation = 0;
        self.best_generation = 0;
        let mut new_population = self.context.random_population_factory();
        self.best_chromosome = new_population.best_chromosome().cloned();

        while !self.is_finished() {
            self.report_round();
            self.current_generation += 1;

            let mut parent_population = new_population;
            let mut child_population = crossover.call(&mut self.context, &parent_population);
            mutate.call(&mut self.context, &mut child_population);
            fitness.call_for_population(&mut child_population);
            child_population.merge(&mut parent_population);
            new_population = compete.call(&mut self.context, child_population);

            new_population.sort();
            self.update_best_chromosome(&new_population);
        }
        self
    }

    fn update_best_chromosome(&mut self, population: &Population<T>) {
        if self.best_chromosome.as_ref() < population.best_chromosome() {
            self.best_chromosome = population.best_chromosome().cloned();
            self.best_generation = self.current_generation;
        }
    }

    fn is_finished(&self) -> bool {
        self.is_finished_by_max_stale_generations() || self.is_finished_by_target_fitness_score()
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.max_stale_generations {
            self.current_generation - self.best_generation >= max_stale_generations
        } else {
            false
        }
    }

    fn is_finished_by_target_fitness_score(&self) -> bool {
        if let Some(target_fitness_score) = self.target_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                fitness_score >= target_fitness_score
            } else {
                false
            }
        } else {
            false
        }
    }

    fn report_round(&self) {
        println!(
            "current generation {}, best fitness score {:?}",
            self.current_generation,
            self.best_fitness_score()
        );
    }

    fn best_fitness_score(&self) -> Option<usize> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }
}

impl<T: Gene, M: Mutate, F: Fitness<T>, S: Crossover, C: Compete> fmt::Display
    for Evolve<T, M, F, S, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "evolve:\n")?;
        write!(
            f,
            "  max_stale_generations: {:?}\n",
            self.max_stale_generations
        )?;
        write!(f, "  mutate: {:?}\n", self.mutate.as_ref())?;
        write!(f, "  fitness: {:?}\n", self.fitness.as_ref())?;
        write!(f, "  crossover: {:?}\n", self.crossover.as_ref())?;
        write!(f, "  compete: {:?}\n", self.compete.as_ref())?;
        write!(
            f,
            "  best_chromosome: {:?}\n",
            self.best_chromosome.as_ref()
        )
    }
}
