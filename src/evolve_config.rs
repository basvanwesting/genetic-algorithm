use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::evolve::Evolve;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct EvolveConfig<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete>
{
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
    EvolveConfig<G, M, F, S, C>
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Evolve<G, M, F, S, C> {
        self.into()
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

    pub fn is_valid(&self) -> bool {
        (self.max_stale_generations.is_some() || self.target_fitness_score.is_some())
            && self.genotype.is_some()
            && self.mutate.is_some()
            && self.fitness.is_some()
            && self.crossover.is_some()
            && self.compete.is_some()
    }
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> Default
    for EvolveConfig<G, M, F, S, C>
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
