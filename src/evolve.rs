use crate::chromosome::Chromosome;
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::evolve_builder::EvolveBuilder;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::population::Population;
use rand::Rng;
use std::fmt;
use std::ops::Range;

pub struct Evolve<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> {
    pub genotype: G,
    pub mutate: M,
    pub fitness: F,
    pub crossover: S,
    pub compete: C,

    pub population_size: usize,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub fitness_ordering: FitnessOrdering,
    pub degeneration_range: Option<Range<f32>>,

    pub current_generation: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<G>>,
    pub population: Population<G>,
    pub degenerate: bool,
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete>
    Evolve<G, M, F, S, C>
{
    pub fn builder() -> EvolveBuilder<G, M, F, S, C> {
        EvolveBuilder::new()
    }

    pub fn call<R: Rng>(mut self, rng: &mut R) -> Self {
        self.degenerate = false;
        self.current_generation = 0;
        self.best_generation = 0;
        self.population = self.population_factory(rng);

        while !self.is_finished() {
            if self.toggle_degenerate() {
                self.population = self.mutate.call(&self.genotype, self.population, rng);
                self.population = self.fitness.call_for_population(self.population);
            } else {
                self.population = self.crossover.call(&self.genotype, self.population, rng);
                self.population = self.mutate.call(&self.genotype, self.population, rng);
                self.population = self.fitness.call_for_population(self.population);
                self.population = self.compete.call(
                    self.population,
                    self.fitness_ordering,
                    self.population_size,
                    rng,
                );
            }

            self.update_best_chromosome();
            //self.report_round();
            self.current_generation += 1;
        }
        self
    }

    fn update_best_chromosome(&mut self) {
        match (
            self.best_chromosome.as_ref(),
            self.population.best_chromosome(self.fitness_ordering),
        ) {
            (None, None) => {}
            (Some(_), None) => {}
            (None, Some(contending_best_chromosome)) => {
                self.best_chromosome = Some(contending_best_chromosome.clone());
                self.best_generation = self.current_generation;
            }
            (Some(current_best_chromosome), Some(contending_best_chromosome)) => {
                match (
                    current_best_chromosome.fitness_score,
                    contending_best_chromosome.fitness_score,
                ) {
                    (None, None) => {}
                    (Some(_), None) => {}
                    (None, Some(_)) => {
                        self.best_chromosome = Some(contending_best_chromosome.clone());
                        self.best_generation = self.current_generation;
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match self.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score > current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    self.best_generation = self.current_generation;
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score < current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    self.best_generation = self.current_generation;
                                }
                            }
                        }
                    }
                }
            }
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
                match self.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= target_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= target_fitness_score,
                }
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
            "current generation: {}, best fitness score: {:?}, fitness score count: {}, fitness score stddev: {}, degenerate: {}",
            self.current_generation,
            self.best_fitness_score(),
            self.population.fitness_score_count(),
            self.population.fitness_score_stddev(),
            self.degenerate,
        );
    }

    pub fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }

    pub fn population_factory<R: Rng>(&mut self, rng: &mut R) -> Population<G> {
        let chromosomes = (0..self.population_size)
            .map(|_| self.genotype.chromosome_factory(rng))
            .collect();
        Population::new(chromosomes)
    }
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete>
    From<EvolveBuilder<G, M, F, S, C>> for Evolve<G, M, F, S, C>
{
    fn from(builder: EvolveBuilder<G, M, F, S, C>) -> Self {
        if !builder.is_valid() {
            panic!("Cannot build Evolve from invalid EvolveBuilder")
        }
        Self {
            genotype: builder.genotype.unwrap(),
            mutate: builder.mutate.unwrap(),
            fitness: builder.fitness.unwrap(),
            crossover: builder.crossover.unwrap(),
            compete: builder.compete.unwrap(),

            population_size: builder.population_size,
            max_stale_generations: builder.max_stale_generations,
            target_fitness_score: builder.target_fitness_score,
            fitness_ordering: builder.fitness_ordering,
            degeneration_range: builder.degeneration_range,

            current_generation: 0,
            best_generation: 0,
            best_chromosome: None,
            population: Population::new_empty(),
            degenerate: false,
        }
    }
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> fmt::Display
    for Evolve<G, M, F, S, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  mutate: {:?}", self.mutate)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;
        writeln!(f, "  crossover: {:?}", self.crossover)?;
        writeln!(f, "  compete: {:?}", self.compete)?;

        writeln!(f, "  population_size: {}", self.population_size)?;
        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  degeneration_range: {:?}", self.degeneration_range)?;

        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
