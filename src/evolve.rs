use crate::chromosome::Chromosome;
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::population::Population;
use rand::Rng;
use std::fmt;
use std::ops::Range;

pub struct Evolve<
    G: Genotype,
    M: Mutate,
    F: Fitness<Genotype = G>,
    S: Crossover,
    C: Compete,
    R: Rng,
> {
    pub genotype: G,
    pub rng: R,
    pub population_size: usize,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub degeneration_range: Option<Range<f32>>,
    pub mutate: Option<M>,
    pub fitness: Option<F>,
    pub crossover: Option<S>,
    pub compete: Option<C>,
    pub current_generation: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<G>>,
    pub population: Population<G>,
    pub degenerate: bool,
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete, R: Rng>
    Evolve<G, M, F, S, C, R>
{
    pub fn new(genotype: G, rng: R) -> Self {
        Self {
            genotype,
            rng,
            population_size: 0,
            max_stale_generations: None,
            target_fitness_score: None,
            degeneration_range: None,
            mutate: None,
            fitness: None,
            crossover: None,
            compete: None,
            current_generation: 0,
            best_generation: 0,
            best_chromosome: None,
            population: Population::new_empty(),
            degenerate: false,
        }
    }

    pub fn with_population_size(mut self, population_size: usize) -> Self {
        self.population_size = population_size;
        self
    }
    pub fn with_max_stale_generations(mut self, max_stale_generations: usize) -> Self {
        self.max_stale_generations = Some(max_stale_generations);
        self
    }
    pub fn with_max_stale_generations_option(
        mut self,
        max_stale_generations_option: Option<usize>,
    ) -> Self {
        self.max_stale_generations = max_stale_generations_option;
        self
    }
    pub fn with_target_fitness_score(mut self, target_fitness_score: FitnessValue) -> Self {
        self.target_fitness_score = Some(target_fitness_score);
        self
    }
    pub fn with_target_fitness_score_option(
        mut self,
        target_fitness_score_option: Option<FitnessValue>,
    ) -> Self {
        self.target_fitness_score = target_fitness_score_option;
        self
    }
    pub fn with_degeneration_range(mut self, degeneration_range: Range<f32>) -> Self {
        if degeneration_range.is_empty() {
            self.degeneration_range = None;
        } else {
            self.degeneration_range = Some(degeneration_range);
        }
        self
    }
    pub fn with_degeneration_range_option(
        mut self,
        degeneration_range_option: Option<Range<f32>>,
    ) -> Self {
        self.degeneration_range = degeneration_range_option;
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

    pub fn with_rng(mut self, rng: R) -> Self {
        self.rng = rng;
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
        let mut fitness = self.fitness.as_ref().cloned().unwrap();
        let crossover = self.crossover.as_ref().cloned().unwrap();
        let compete = self.compete.as_ref().cloned().unwrap();

        self.degenerate = false;
        self.current_generation = 0;
        self.best_generation = 0;
        self.population = self.population_factory();
        self.best_chromosome = self.population.best_chromosome().cloned();

        while !self.is_finished() {
            if self.toggle_degenerate() {
                self.population = mutate.call(&self.genotype, self.population, &mut self.rng);
                self.population = fitness.call_for_population(self.population);
            } else {
                self.population = crossover.call(&self.genotype, self.population, &mut self.rng);
                self.population = mutate.call(&self.genotype, self.population, &mut self.rng);
                self.population = fitness.call_for_population(self.population);
                self.population =
                    compete.call(self.population, self.population_size, &mut self.rng);
            }

            self.update_best_chromosome();
            //self.report_round();
            self.current_generation += 1;
        }
        self
    }

    fn update_best_chromosome(&mut self) {
        if self.best_chromosome.as_ref() < self.population.best_chromosome() {
            self.best_chromosome = self.population.best_chromosome().cloned();
            self.best_generation = self.current_generation;
        }
    }

    fn toggle_degenerate(&mut self) -> bool {
        if let Some(degeneration_range) = self.degeneration_range.as_ref() {
            let fitness_score_stddev = self.population.fitness_score_stddev();
            if self.degenerate && fitness_score_stddev > degeneration_range.end {
                self.degenerate = false;
            } else if !self.degenerate && fitness_score_stddev < degeneration_range.start {
                self.degenerate = true;
            }
        }
        self.degenerate
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

    #[allow(dead_code)]
    fn report_round(&self) {
        println!(
            "current generation: {}, best fitness score: {:?}, fitness score stddev: {}, degenerate: {}",
            self.current_generation,
            self.best_fitness_score(),
            self.population.fitness_score_stddev(),
            self.degenerate,
        );
    }

    pub fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }

    pub fn population_factory(&mut self) -> Population<G> {
        let chromosomes = (0..self.population_size)
            .map(|_| self.genotype.chromosome_factory(&mut self.rng))
            .collect();
        Population::new(chromosomes)
    }
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete, R: Rng>
    fmt::Display for Evolve<G, M, F, S, C, R>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve:")?;
        writeln!(f, "  population_size: {}", self.population_size)?;
        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  degeneration_range: {:?}", self.degeneration_range)?;
        writeln!(f, "  mutate: {:?}", self.mutate.as_ref())?;
        writeln!(f, "  fitness: {:?}", self.fitness.as_ref())?;
        writeln!(f, "  crossover: {:?}", self.crossover.as_ref())?;
        writeln!(f, "  compete: {:?}", self.compete.as_ref())?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
