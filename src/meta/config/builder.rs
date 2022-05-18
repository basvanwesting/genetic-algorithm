use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::Genotype;
use crate::meta::config::Config;
use crate::mutate::MutateDispatch;
use crate::strategy::evolve::EvolveBuilder;
use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

#[derive(Clone, Debug)]
pub struct Builder<G: Genotype, F: Fitness<Genotype = G>> {
    pub evolve_builder:
        Option<EvolveBuilder<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch>>,
    pub evolve_fitness_to_micro_second_factor: FitnessValue,
    pub rounds: usize,
    pub population_sizes: Vec<usize>,
    pub max_stale_generations_options: Vec<Option<usize>>,
    pub target_fitness_score_options: Vec<Option<FitnessValue>>,
    pub degeneration_range_options: Vec<Option<Range<f32>>>,
    pub mutates: Vec<MutateDispatch>,
    pub crossovers: Vec<CrossoverDispatch>,
    pub competes: Vec<CompeteDispatch>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> Builder<G, F> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<Config<G, F>, TryFromBuilderError> {
        self.try_into()
    }

    pub fn with_evolve_builder(
        mut self,
        evolve_builder: EvolveBuilder<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch>,
    ) -> Self {
        self.evolve_builder = Some(evolve_builder);
        self
    }
    pub fn with_rounds(mut self, rounds: usize) -> Self {
        self.rounds = rounds;
        self
    }
    pub fn with_evolve_fitness_to_micro_second_factor(
        mut self,
        evolve_fitness_to_micro_second_factor: FitnessValue,
    ) -> Self {
        self.evolve_fitness_to_micro_second_factor = evolve_fitness_to_micro_second_factor;
        self
    }
    pub fn with_population_sizes(mut self, population_sizes: Vec<usize>) -> Self {
        self.population_sizes = population_sizes;
        self
    }
    pub fn with_max_stale_generations_options(
        mut self,
        max_stale_generations_options: Vec<Option<usize>>,
    ) -> Self {
        self.max_stale_generations_options = max_stale_generations_options;
        self
    }
    pub fn with_target_fitness_score_options(
        mut self,
        target_fitness_score_options: Vec<Option<FitnessValue>>,
    ) -> Self {
        self.target_fitness_score_options = target_fitness_score_options;
        self
    }
    pub fn with_degeneration_range_options(
        mut self,
        degeneration_range_options: Vec<Option<Range<f32>>>,
    ) -> Self {
        self.degeneration_range_options = degeneration_range_options;
        self
    }
    pub fn with_mutates(mut self, mutates: Vec<MutateDispatch>) -> Self {
        self.mutates = mutates;
        self
    }
    pub fn with_crossovers(mut self, crossovers: Vec<CrossoverDispatch>) -> Self {
        self.crossovers = crossovers;
        self
    }
    pub fn with_competes(mut self, competes: Vec<CompeteDispatch>) -> Self {
        self.competes = competes;
        self
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> Default for Builder<G, F> {
    fn default() -> Self {
        Self {
            evolve_builder: None,
            evolve_fitness_to_micro_second_factor: 1_000_000,
            rounds: 0,
            population_sizes: vec![],
            max_stale_generations_options: vec![None],
            target_fitness_score_options: vec![None],
            degeneration_range_options: vec![None],
            mutates: vec![],
            crossovers: vec![],
            competes: vec![],
        }
    }
}
