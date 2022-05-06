use crate::chromosome::Chromosome;
use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::evolve_builder::EvolveBuilder;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::meta::config_builder::ConfigBuilder;
use crate::mutate::MutateDispatch;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Config<G: Genotype, F: Fitness<Genotype = G>> {
    pub evolve_builder: EvolveBuilder<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch>,
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
    pub fn builder() -> ConfigBuilder<G, F> {
        ConfigBuilder::new()
    }

    // order matters so keep close to build_genotype
    pub fn evolve_builder_for_chromosome(
        &self,
        chromosome: &Chromosome<MultiIndexGenotype>,
    ) -> EvolveBuilder<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch> {
        let genes = &chromosome.genes;

        self.evolve_builder
            .clone()
            .with_population_size(self.population_sizes[genes[0]])
            .with_max_stale_generations_option(self.max_stale_generations_options[genes[1]])
            .with_target_fitness_score_option(self.target_fitness_score_options[genes[2]])
            .with_degeneration_range_option(self.degeneration_range_options[genes[3]].clone())
            .with_mutate(self.mutates[genes[4]].clone())
            .with_crossover(self.crossovers[genes[5]].clone())
            .with_compete(self.competes[genes[6]].clone())
    }

    // order matters so keep close to evolve_builder_for_chromosome
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

impl<G: Genotype, F: Fitness<Genotype = G>> From<ConfigBuilder<G, F>> for Config<G, F> {
    fn from(builder: ConfigBuilder<G, F>) -> Self {
        if !builder.is_valid() {
            panic!("Cannot build Meta::Config from invalid Meta::ConfigBuilder")
        }
        Self {
            evolve_builder: builder.evolve_builder.unwrap(),
            evolve_fitness_to_micro_second_factor: builder.evolve_fitness_to_micro_second_factor,
            rounds: builder.rounds,
            population_sizes: builder.population_sizes,
            max_stale_generations_options: builder.max_stale_generations_options,
            target_fitness_score_options: builder.target_fitness_score_options,
            degeneration_range_options: builder.degeneration_range_options,
            mutates: builder.mutates,
            crossovers: builder.crossovers,
            competes: builder.competes,
        }
    }
}
