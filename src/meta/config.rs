use crate::chromosome::Chromosome;
use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::evolve_config::EvolveConfig;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::mutate::MutateDispatch;
use rand::Rng;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Config<G: Genotype, F: Fitness<Genotype = G>, R: Rng> {
    pub evolve_config: EvolveConfig<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch, R>,
    pub rounds: usize,
    pub population_sizes: Vec<usize>,
    pub max_stale_generations_options: Vec<Option<usize>>,
    pub target_fitness_score_options: Vec<Option<FitnessValue>>,
    pub degeneration_range_options: Vec<Option<Range<f32>>>,
    pub mutates: Vec<MutateDispatch>,
    pub crossovers: Vec<CrossoverDispatch>,
    pub competes: Vec<CompeteDispatch>,
}

impl<G: Genotype, F: Fitness<Genotype = G>, R: Rng> Config<G, F, R> {
    pub fn new(
        evolve_config: EvolveConfig<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch, R>,
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
    ) -> EvolveConfig<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch, R> {
        let genes = &chromosome.genes;

        let mut evolve_config = self.evolve_config.clone();
        evolve_config.population_size = self.population_sizes[genes[0]];
        evolve_config.max_stale_generations = self.max_stale_generations_options[genes[1]];
        evolve_config.target_fitness_score = self.target_fitness_score_options[genes[2]];
        evolve_config.degeneration_range = self.degeneration_range_options[genes[3]].clone();
        evolve_config.mutate = Some(self.mutates[genes[4]].clone());
        evolve_config.crossover = Some(self.crossovers[genes[5]].clone());
        evolve_config.compete = Some(self.competes[genes[6]].clone());

        evolve_config
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
