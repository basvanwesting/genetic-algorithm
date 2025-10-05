//! A solution strategy for finding the best chromosome in case of small problem spaces (with a 100% guarantee)
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as PermutateBuilder, TryFromBuilderError as TryFromPermutateBuilderError,
};

use super::{
    Strategy, StrategyAction, StrategyConfig, StrategyReporter, StrategyReporterNoop,
    StrategyState, StrategyVariant,
};
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::{MutationType, PermutateGenotype};
use crate::population::Population;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

pub use self::reporter::Simple as PermutateReporterSimple;
pub use crate::strategy::reporter::Duration as PermutateReporterDuration;
pub use crate::strategy::reporter::Noop as PermutateReporterNoop;

#[derive(Copy, Clone, Debug, Default)]
pub enum PermutateVariant {
    #[default]
    Standard,
}

/// All possible combinations of genes are iterated over as chromosomes (unless the genotype seeds
/// specific genes, in which case only those genes are iterated over). The fitness is calculated
/// for each chromosome and the best is taken. For efficiency reasons the full population is never
/// instantiated as a whole.
///
/// The `chromosome_permutations_size` is subject to combinatorial explosion, so check the genotype
/// for practical values before using the [Permutate] strategy. This will not pose any memory
/// issues, as the permutations are not instantiated at the same time, just iterated over. But it
/// will take forever...
///
/// There is a method to permutate
/// [RangeGenotype](crate::genotype::RangeGenotype) and
/// [MultiRangeGenotype](crate::genotype::MultiRangeGenotype) chromosomes. Listed in
/// descending priority:
/// * With allele_mutation_scaled_range(s) set on genotype:
///     * First scale (index = 0) traverses the whole allele_range(s) with the upper bound of the
///     first scale as step size.
///     * Other scales (index > 0) center around the best chromomsome of the previous scale,
///     traversing the previous scale bounds around the best chromosome with the upper bound of the
///     current scale as step size.
///     * Scale down after grid is fully traversed
/// * With allele_mutation_range(s) set on genotype: Permutation not supported
/// * With only allele_range(s) set on genotype:  Permutation not supported
///
/// There are reporting hooks in the loop receiving the [PermutateState], which can by handled by an
/// [StrategyReporter] (e.g. [PermutateReporterDuration], [PermutateReporterSimple]). But you are encouraged to
/// roll your own, see [StrategyReporter].
///
/// See [PermutateBuilder] for initialization options.
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// Example:
/// ```ignore
/// use genetic_algorithm::strategy::permutate::prelude::*;
/// use genetic_algorithm::fitness::placeholders::CountStaticTrue;
///
/// // the search space
/// let genotype = StaticBinaryGenotype::<12, 100>::builder() // boolean alleles
///     .with_genes_size(12)                 // 12 genes per chromosome
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let permutate = Permutate::builder()
///     .with_genotype(genotype)
///     .with_fitness(CountStaticTrue)                    // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_reporter(PermutateReporterSimple::new(100)) // optional builder step, report every 100 generations
///     .call()
///     .unwrap();
///
/// // it's all about the best genes after all
/// let (best_genes, best_fitness_score) = permutate.best_genes_and_fitness_score().unwrap();
/// assert_eq!(best_genes, vec![false; 12]);
/// assert_eq!(best_fitness_score, 0);
/// ```
pub struct Permutate<
    G: PermutateGenotype,
    F: Fitness<Genotype = G>,
    SR: StrategyReporter<Genotype = G>,
> {
    pub genotype: G,
    pub fitness: F,
    pub config: PermutateConfig,
    pub state: PermutateState,
    pub reporter: SR,
}

pub struct PermutateConfig {
    pub variant: PermutateVariant,
    pub fitness_ordering: FitnessOrdering,
    pub replace_on_equal_fitness: bool,
    pub population_window_size: usize,
}

/// Stores the state of the Permutate strategy
pub struct PermutateState {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub stale_generations: usize,
    pub scale_generation: usize,
    pub best_generation: usize,
    pub best_fitness_score: Option<FitnessValue>,
    pub durations: HashMap<StrategyAction, Duration>,
    pub chromosome: Option<Chromosome>,
    pub population: Population,
    pub current_scale_index: Option<usize>,
}

impl<G: PermutateGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>> Strategy<G>
    for Permutate<G, F, SR>
{
    fn call(&mut self) {
        let now = Instant::now();
        self.reporter
            .on_enter(&self.genotype, &self.state, &self.config);
        self.setup();
        self.reporter
            .on_start(&self.genotype, &self.state, &self.config);
        while !self.is_finished() {
            self.call_population_based();
            self.state.scale(&self.genotype, &self.config);
        }
        self.reporter
            .on_finish(&self.genotype, &self.state, &self.config);
        self.cleanup();
        self.state.close_duration(now.elapsed());
        self.reporter
            .on_exit(&self.genotype, &self.state, &self.config);
    }
    fn best_generation(&self) -> usize {
        self.state.best_generation
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.state.best_fitness_score()
    }
    fn best_genes(&self) -> Option<G::Genes> {
        if self.state.best_fitness_score().is_some() {
            Some(self.genotype.best_genes().clone())
        } else {
            None
        }
    }
    fn flush_reporter(&mut self, output: &mut Vec<u8>) {
        self.reporter.flush(output);
    }
}

impl<G: PermutateGenotype, F: Fitness<Genotype = G>> Permutate<G, F, StrategyReporterNoop<G>> {
    pub fn builder() -> PermutateBuilder<G, F, StrategyReporterNoop<G>> {
        PermutateBuilder::new()
    }
}

impl<G: PermutateGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    Permutate<G, F, SR>
{
    pub fn setup(&mut self) {
        let now = Instant::now();
        // Initialize with first population window
        let first_population = self
            .genotype
            .clone()
            .population_permutations_into_iter(
                self.config.population_window_size,
                self.state.current_scale_index,
            )
            .next();

        if let Some(population) = first_population {
            self.state.population = population;
            self.fitness
                .call_for_state_population(&self.genotype, &mut self.state, &self.config);
            self.state.update_best_population_and_report(
                &mut self.genotype,
                &self.config,
                &mut self.reporter,
            );
        }
        self.state
            .add_duration(StrategyAction::SetupAndCleanup, now.elapsed());
    }
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.state.chromosome.take();
        std::mem::take(&mut self.state.population.chromosomes);
        self.genotype.chromosomes_cleanup();
        self.state
            .add_duration(StrategyAction::SetupAndCleanup, now.elapsed());
    }
    fn is_finished(&self) -> bool {
        self.is_finished_by_max_scale_generation()
    }
    fn is_finished_by_max_scale_generation(&self) -> bool {
        self.state.scale_generation > 0
    }

    fn call_population_based(&mut self) {
        self.genotype
            .clone()
            .population_permutations_into_iter(
                self.config.population_window_size,
                self.state.current_scale_index,
            )
            .for_each(|population| {
                self.state.increment_generation();
                self.state.population = population;
                self.fitness.call_for_state_population(
                    &self.genotype,
                    &mut self.state,
                    &self.config,
                );
                self.state.update_best_population_and_report(
                    &mut self.genotype,
                    &self.config,
                    &mut self.reporter,
                );
                self.reporter
                    .on_new_generation(&self.genotype, &self.state, &self.config);
            });
    }
}

impl StrategyConfig for PermutateConfig {
    fn fitness_ordering(&self) -> FitnessOrdering {
        self.fitness_ordering
    }
    fn replace_on_equal_fitness(&self) -> bool {
        self.replace_on_equal_fitness
    }
    fn variant(&self) -> StrategyVariant {
        StrategyVariant::Permutate(self.variant)
    }
}

impl StrategyState for PermutateState {
    fn chromosome_as_ref(&self) -> &Option<Chromosome> {
        &self.chromosome
    }
    fn population_as_ref(&self) -> &Population {
        &self.population
    }
    fn chromosome_as_mut(&mut self) -> &mut Option<Chromosome> {
        &mut self.chromosome
    }
    fn population_as_mut(&mut self) -> &mut Population {
        &mut self.population
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_fitness_score
    }
    fn best_generation(&self) -> usize {
        self.best_generation
    }
    fn current_generation(&self) -> usize {
        self.current_generation
    }
    fn current_iteration(&self) -> usize {
        self.current_iteration
    }
    fn increment_generation(&mut self) {
        self.current_generation += 1;
        self.scale_generation += 1;
    }
    fn stale_generations(&self) -> usize {
        self.stale_generations
    }
    fn increment_stale_generations(&mut self) {
        self.stale_generations += 1;
    }
    fn reset_stale_generations(&mut self) {
        self.stale_generations = 0;
    }
    fn scale_generation(&self) -> usize {
        self.scale_generation
    }
    fn reset_scale_generation(&mut self) {
        self.scale_generation = 0;
    }
    fn current_scale_index(&self) -> Option<usize> {
        self.current_scale_index
    }
    fn population_cardinality(&self) -> Option<usize> {
        None
    }
    fn durations(&self) -> &HashMap<StrategyAction, Duration> {
        &self.durations
    }
    fn add_duration(&mut self, action: StrategyAction, duration: Duration) {
        *self.durations.entry(action).or_default() += duration;
    }
    fn total_duration(&self) -> Duration {
        self.durations.values().sum()
    }
}

impl PermutateState {
    fn update_best_population_and_report<
        G: PermutateGenotype,
        SR: StrategyReporter<Genotype = G>,
    >(
        &mut self,
        genotype: &mut G,
        config: &PermutateConfig,
        reporter: &mut SR,
    ) {
        let now = Instant::now();
        // Find the best chromosome in the current population
        if let Some(best_in_population) = self.population.best_chromosome(config.fitness_ordering) {
            match self.is_better_chromosome(
                best_in_population,
                &config.fitness_ordering,
                config.replace_on_equal_fitness,
            ) {
                (true, true) => {
                    self.best_generation = self.current_generation;
                    self.best_fitness_score = best_in_population.fitness_score();
                    genotype.save_best_genes(best_in_population);
                    reporter.on_new_best_chromosome(genotype, self, config);
                    self.reset_stale_generations();
                }
                (true, false) => {
                    genotype.save_best_genes(best_in_population);
                    reporter.on_new_best_chromosome_equal_fitness(genotype, self, config);
                    self.increment_stale_generations()
                }
                _ => self.increment_stale_generations(),
            }
        } else {
            self.increment_stale_generations();
        }
        self.add_duration(StrategyAction::UpdateBestChromosome, now.elapsed());
    }
    fn scale<G: PermutateGenotype>(&mut self, genotype: &G, _config: &PermutateConfig) {
        if let Some(current_scale_index) = self.current_scale_index {
            if let Some(max_scale_index) = genotype.max_scale_index() {
                if current_scale_index < max_scale_index {
                    self.current_scale_index = Some(current_scale_index + 1);
                    self.reset_scale_generation();
                    self.reset_stale_generations();
                }
            }
        }
    }
}

impl<G: PermutateGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    TryFrom<PermutateBuilder<G, F, SR>> for Permutate<G, F, SR>
{
    type Error = TryFromPermutateBuilderError;

    fn try_from(builder: PermutateBuilder<G, F, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromPermutateBuilderError(
                "Permutate requires a PermutateGenotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromPermutateBuilderError("Permutate requires a Fitness"))
        } else if builder
            .genotype
            .as_ref()
            .map(|o| !o.mutation_type_allows_permutation())
            .unwrap()
        {
            Err(TryFromPermutateBuilderError(
                "The Genotype's mutation_type does not allow permutation",
            ))
        } else if builder.population_window_size == 0 {
            Err(TryFromPermutateBuilderError(
                "Permutate requires a population_window_size > 0",
            ))
        } else {
            let genotype = builder.genotype.unwrap();
            let state = PermutateState::new(&genotype);

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),

                config: PermutateConfig {
                    fitness_ordering: builder.fitness_ordering,
                    replace_on_equal_fitness: builder.replace_on_equal_fitness,
                    population_window_size: builder.population_window_size,
                    ..Default::default()
                },
                state,
                reporter: builder.reporter,
            })
        }
    }
}

impl Default for PermutateConfig {
    fn default() -> Self {
        Self {
            variant: Default::default(),
            fitness_ordering: FitnessOrdering::Maximize,
            replace_on_equal_fitness: false,
            population_window_size: 100,
        }
    }
}
impl PermutateConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PermutateState {
    pub fn new<G: PermutateGenotype>(genotype: &G) -> Self {
        let base = Self {
            current_iteration: 0,
            current_generation: 0,
            stale_generations: 0,
            scale_generation: 0,
            current_scale_index: None,
            best_generation: 0,
            best_fitness_score: None,
            chromosome: None,
            population: Population::new_empty(),
            durations: HashMap::new(),
        };
        match genotype.mutation_type() {
            MutationType::Scaled => Self {
                current_scale_index: Some(0),
                ..base
            },
            MutationType::Relative => base,
            MutationType::Random => base,
        }
    }
}

impl<G: PermutateGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    fmt::Display for Permutate<G, F, SR>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate:")?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;
        writeln!(f)?;

        writeln!(f, "{}", self.config)?;
        writeln!(f, "{}", self.state)?;
        writeln!(f, "{}", self.genotype)
    }
}

impl fmt::Display for PermutateConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_config:")?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)
    }
}

impl fmt::Display for PermutateState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_state:")?;
        writeln!(f, "  current iteration: -")?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  current scale index: {:?}", self.current_scale_index)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())
    }
}
