use super::Evolve;
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::strategy::Strategy;
use rand::Rng;
use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for an Evolve struct.
#[derive(Clone, Debug)]
pub struct Builder<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> {
    pub genotype: Option<G>,
    pub population_size: usize,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub fitness_ordering: FitnessOrdering,
    pub degeneration_range: Option<Range<f32>>,
    pub mutate: Option<M>,
    pub fitness: Option<F>,
    pub crossover: Option<S>,
    pub compete: Option<C>,
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete>
    Builder<G, M, F, S, C>
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Evolve<G, M, F, S, C>, TryFromBuilderError> {
        self.try_into()
    }

    pub fn call<R: Rng>(self, rng: &mut R) -> Result<Evolve<G, M, F, S, C>, TryFromBuilderError> {
        let mut evolve: Evolve<G, M, F, S, C> = self.try_into()?;
        evolve.call(rng);
        Ok(evolve)
    }
    pub fn call_repeatedly<R: Rng>(
        self,
        max_repeats: usize,
        rng: &mut R,
    ) -> Result<Evolve<G, M, F, S, C>, TryFromBuilderError> {
        let mut best_evolve: Option<Evolve<G, M, F, S, C>> = None;
        for iteration in 0..max_repeats {
            let mut contending_run: Evolve<G, M, F, S, C> = self.clone().try_into()?;
            contending_run.current_iteration = iteration;
            contending_run.call(rng);
            if contending_run.is_finished_by_target_fitness_score() {
                best_evolve = Some(contending_run);
                break;
            }
            if let Some(best_run) = best_evolve.as_ref() {
                match (
                    best_run.best_fitness_score(),
                    contending_run.best_fitness_score(),
                ) {
                    (None, None) => {}
                    (Some(_), None) => {}
                    (None, Some(_)) => {
                        best_evolve = Some(contending_run);
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match contending_run.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score >= current_fitness_score {
                                    best_evolve = Some(contending_run);
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score <= current_fitness_score {
                                    best_evolve = Some(contending_run);
                                }
                            }
                        }
                    }
                }
            } else {
                best_evolve = Some(contending_run);
            }
        }
        Ok(best_evolve.unwrap())
    }

    pub fn with_genotype(mut self, genotype: G) -> Self {
        self.genotype = Some(genotype);
        self
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
    pub fn with_fitness_ordering(mut self, fitness_ordering: FitnessOrdering) -> Self {
        self.fitness_ordering = fitness_ordering;
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
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> Default
    for Builder<G, M, F, S, C>
{
    fn default() -> Self {
        Self {
            genotype: None,
            population_size: 0,
            max_stale_generations: None,
            target_fitness_score: None,
            fitness_ordering: FitnessOrdering::Maximize,
            degeneration_range: None,
            mutate: None,
            fitness: None,
            crossover: None,
            compete: None,
        }
    }
}
