use crate::chromosome::Chromosome;
use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::fitness::Fitness;
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::meta::MetaEvolveConfig;
use crate::mutate::MutateDispatch;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Config<G: Genotype, F: Fitness<Genotype = G>> {
    pub rounds: usize,
    pub evolve_genotype: G,
    pub evolve_fitness: F,
    pub population_sizes: Vec<usize>,
    pub max_stale_generations_options: Vec<Option<usize>>,
    pub target_fitness_score_options: Vec<Option<isize>>,
    pub degeneration_range_options: Vec<Option<Range<f32>>>,
    pub mutates: Vec<MutateDispatch>,
    pub crossovers: Vec<CrossoverDispatch>,
    pub competes: Vec<CompeteDispatch>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> Config<G, F> {
    pub fn new(
        rounds: usize,
        evolve_genotype: G,
        evolve_fitness: F,
        population_sizes: Vec<usize>,
        max_stale_generations_options: Vec<Option<usize>>,
        target_fitness_score_options: Vec<Option<isize>>,
        degeneration_range_options: Vec<Option<Range<f32>>>,
        mutates: Vec<MutateDispatch>,
        crossovers: Vec<CrossoverDispatch>,
        competes: Vec<CompeteDispatch>,
    ) -> Self {
        Self {
            rounds,
            evolve_genotype,
            evolve_fitness,
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
            && self.population_sizes.len() > 0
            && self.max_stale_generations_options.len() > 0
            && self.target_fitness_score_options.len() > 0
            && self.degeneration_range_options.len() > 0
            && self.mutates.len() > 0
            && self.crossovers.len() > 0
            && self.competes.len() > 0
    }

    // order matters so keep close to build_genotype
    pub fn evolve_config_for_chromosome(
        &self,
        chromosome: &Chromosome<MultiIndexGenotype>,
    ) -> MetaEvolveConfig {
        let genes = &chromosome.genes;

        MetaEvolveConfig {
            population_size: self.population_sizes[genes[0]],
            max_stale_generations_option: self.max_stale_generations_options[genes[1]],
            target_fitness_score_option: self.target_fitness_score_options[genes[2]],
            degeneration_range_option: self.degeneration_range_options[genes[3]].clone(),
            mutate: self.mutates[genes[4]].clone(),
            crossover: self.crossovers[genes[5]].clone(),
            compete: self.competes[genes[6]].clone(),
        }
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
