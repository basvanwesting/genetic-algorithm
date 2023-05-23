mod builder;

pub use self::builder::{
    Builder as ConfigBuilder, TryFromBuilderError as TryFromConfigBuilderError,
};

use crate::chromosome::Chromosome;
use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::extension::ExtensionDispatch;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{Genotype, MultiDiscreteGenotype};
use crate::mutate::MutateDispatch;
use crate::strategy::evolve::EvolveBuilder;

#[derive(Clone, Debug)]
pub struct Config<G: Genotype, F: Fitness<Genotype = G>> {
    pub evolve_builder:
        EvolveBuilder<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch, ExtensionDispatch>,
    pub evolve_fitness_to_micro_second_factor: FitnessValue,
    pub rounds: usize,
    pub target_population_sizes: Vec<usize>,
    pub max_stale_generations_options: Vec<Option<usize>>,
    pub max_chromosome_age_options: Vec<Option<usize>>,
    pub target_fitness_score_options: Vec<Option<FitnessValue>>,
    pub mutates: Vec<MutateDispatch>,
    pub crossovers: Vec<CrossoverDispatch>,
    pub competes: Vec<CompeteDispatch>,
    pub extensions: Vec<ExtensionDispatch>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> Config<G, F> {
    pub fn builder() -> ConfigBuilder<G, F> {
        ConfigBuilder::new()
    }

    // order matters so keep close to build_genotype
    pub fn evolve_builder_for_chromosome(
        &self,
        chromosome: &Chromosome<MultiDiscreteGenotype>,
    ) -> EvolveBuilder<G, MutateDispatch, F, CrossoverDispatch, CompeteDispatch, ExtensionDispatch>
    {
        let genes = &chromosome.genes;

        self.evolve_builder
            .clone()
            .with_target_population_size(self.target_population_sizes[genes[0]])
            .with_mutate(self.mutates[genes[1]].clone())
            .with_crossover(self.crossovers[genes[2]].clone())
            .with_compete(self.competes[genes[3]].clone())
            .with_extension(self.extensions[genes[4]].clone())
            .with_max_stale_generations_option(self.max_stale_generations_options[genes[5]])
            .with_max_chromosome_age_option(self.max_chromosome_age_options[genes[6]])
            .with_target_fitness_score_option(self.target_fitness_score_options[genes[7]])
    }

    // order matters so keep close to evolve_builder_for_chromosome
    pub fn build_genotype(&self) -> MultiDiscreteGenotype {
        MultiDiscreteGenotype::builder()
            .with_allele_lists(vec![
                (0..self.target_population_sizes.len()).collect(),
                (0..self.mutates.len()).collect(),
                (0..self.crossovers.len()).collect(),
                (0..self.competes.len()).collect(),
                (0..self.extensions.len()).collect(),
                (0..self.max_stale_generations_options.len()).collect(),
                (0..self.max_chromosome_age_options.len()).collect(),
                (0..self.target_fitness_score_options.len()).collect(),
            ])
            .build()
            .unwrap()
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> TryFrom<ConfigBuilder<G, F>> for Config<G, F> {
    type Error = TryFromConfigBuilderError;

    fn try_from(builder: ConfigBuilder<G, F>) -> Result<Self, Self::Error> {
        if builder.evolve_builder.is_none() {
            Err(TryFromConfigBuilderError(
                "MetaConfig requires an EvolveBuilder",
            ))
        } else if builder
            .evolve_builder
            .as_ref()
            .map(|e| e.genotype.is_none())
            .unwrap()
        {
            Err(TryFromConfigBuilderError(
                "MetaConfig's EvolveBuilder requires a Genotype",
            ))
        } else if builder
            .evolve_builder
            .as_ref()
            .map(|e| e.fitness.is_none())
            .unwrap()
        {
            Err(TryFromConfigBuilderError(
                "MetaConfig's EvolveBuilder requires a Fitness",
            ))
        } else if builder.target_population_sizes.is_empty() {
            Err(TryFromConfigBuilderError(
                "MetaConfig requires at least one target_population_size",
            ))
        } else if builder
            .max_stale_generations_options
            .iter()
            .all(|o| o.is_none())
            && builder
                .target_fitness_score_options
                .iter()
                .all(|o| o.is_none())
        {
            Err(TryFromConfigBuilderError(
                "MetaConfig requires at least one max_stale_generations_option or target_fitness_score_option that is not None",
            ))
        } else if builder.mutates.is_empty() {
            Err(TryFromConfigBuilderError(
                "MetaConfig requires at least one Mutate strategy",
            ))
        } else if builder.extensions.is_empty() {
            Err(TryFromConfigBuilderError(
                "MetaConfig requires at least one Extension strategy",
            ))
        } else if builder.crossovers.is_empty() {
            Err(TryFromConfigBuilderError(
                "MetaConfig requires at least one Crossover strategy",
            ))
        } else if builder.competes.is_empty() {
            Err(TryFromConfigBuilderError(
                "MetaConfig requires at least one Compete strategy",
            ))
        } else {
            Ok(Self {
                evolve_builder: builder.evolve_builder.unwrap(),
                evolve_fitness_to_micro_second_factor: builder
                    .evolve_fitness_to_micro_second_factor,
                rounds: builder.rounds,
                target_population_sizes: builder.target_population_sizes,
                max_stale_generations_options: builder.max_stale_generations_options,
                max_chromosome_age_options: builder.max_chromosome_age_options,
                target_fitness_score_options: builder.target_fitness_score_options,
                mutates: builder.mutates,
                crossovers: builder.crossovers,
                competes: builder.competes,
                extensions: builder.extensions,
            })
        }
    }
}
