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
    pub evolve_config: EvolveConfig<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch>,
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
    pub fn new(
        evolve_config: EvolveConfig<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch>,
        evolve_fitness_to_micro_second_factor: FitnessValue,
        rounds: usize,
        population_sizes: Vec<usize>,
        max_stale_generations_options: Vec<Option<usize>>,
        target_fitness_score_options: Vec<Option<FitnessValue>>,
        degeneration_range_options: Vec<Option<Range<f32>>>,
        mutates: Vec<MutateDispatch>,
        crossovers: Vec<CrossoverDispatch>,
        competes: Vec<CompeteDispatch>,
    ) -> Self {
        Self {
            evolve_config,
            evolve_fitness_to_micro_second_factor,
            rounds,
            population_sizes,
            max_stale_generations_options,
            target_fitness_score_options,
            degeneration_range_options,
            mutates,
            crossovers,
            competes,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.rounds > 0
            && self.evolve_config.genotype.is_some()
            && self.evolve_config.fitness.is_some()
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
