use crate::chromosome::Chromosome;
use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::evolve_config::EvolveConfig;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::mutate::MutateDispatch;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Config<G: Genotype, F: Fitness<Genotype = G>> {
    pub evolve_config:
        Option<EvolveConfig<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch>>,
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

impl<G: Genotype, F: Fitness<Genotype = G>> Config<G, F> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_evolve_config(
        mut self,
        evolve_config: EvolveConfig<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch>,
    ) -> Self {
        self.evolve_config = Some(evolve_config);
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

    // TODO: remove clones for genotype/fitness check
    pub fn is_valid(&self) -> bool {
        self.rounds > 0
            && self.evolve_config.is_some()
            && self.evolve_config.clone().map(|c| c.genotype).is_some()
            && self.evolve_config.clone().map(|c| c.fitness).is_some()
            && !self.population_sizes.is_empty()
            && !self.max_stale_generations_options.is_empty()
            && !self.target_fitness_score_options.is_empty()
            && !self.degeneration_range_options.is_empty()
            && !self.mutates.is_empty()
            && !self.crossovers.is_empty()
            && !self.competes.is_empty()
    }

    // order matters so keep close to build_genotype
    pub fn evolve_config_for_chromosome(
        &self,
        chromosome: &Chromosome<MultiIndexGenotype>,
    ) -> EvolveConfig<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch> {
        let genes = &chromosome.genes;

        self.evolve_config
            .clone()
            .unwrap()
            .with_population_size(self.population_sizes[genes[0]])
            .with_max_stale_generations_option(self.max_stale_generations_options[genes[1]])
            .with_target_fitness_score_option(self.target_fitness_score_options[genes[2]])
            .with_degeneration_range_option(self.degeneration_range_options[genes[3]].clone())
            .with_mutate(self.mutates[genes[4]].clone())
            .with_crossover(self.crossovers[genes[5]].clone())
            .with_compete(self.competes[genes[6]].clone())
    }

    // order matters so keep close to evolve_config_for_chromosome
    pub fn build_genotype(&self) -> MultiIndexGenotype {
        MultiIndexGenotype::new()
            .with_gene_value_sizes(vec![
                self.population_sizes.len(),
                self.max_stale_generations_options.len(),
                self.target_fitness_score_options.len(),
                self.degeneration_range_options.len(),
                self.mutates.len(),
                self.crossovers.len(),
                self.competes.len(),
            ])
            .build()
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> Default for Config<G, F> {
    fn default() -> Self {
        Self {
            evolve_config: None,
            evolve_fitness_to_micro_second_factor: 1_000_000,
            rounds: 0,
            population_sizes: vec![],
            max_stale_generations_options: vec![],
            target_fitness_score_options: vec![],
            degeneration_range_options: vec![],
            mutates: vec![],
            crossovers: vec![],
            competes: vec![],
        }
    }
}
