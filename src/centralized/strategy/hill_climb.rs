//! A solution strategy for finding the best chromosome, when search space is convex with little local optima or crossover is impossible or inefficient
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as HillClimbBuilder, TryFromBuilderError as TryFromHillClimbBuilderError,
};

use super::{
    Strategy, StrategyAction, StrategyConfig, StrategyReporter, StrategyReporterNoop,
    StrategyState, StrategyVariant,
};
use crate::centralized::chromosome::Chromosome;
use crate::centralized::fitness::{Fitness, FitnessCache, FitnessOrdering, FitnessValue};
use crate::centralized::genotype::{HillClimbGenotype, MutationType};
use crate::centralized::population::Population;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

pub use self::reporter::Simple as HillClimbReporterSimple;
pub use crate::centralized::strategy::reporter::Duration as HillClimbReporterDuration;
pub use crate::centralized::strategy::reporter::Noop as HillClimbReporterNoop;

#[derive(Copy, Clone, Debug, Default)]
pub enum HillClimbVariant {
    #[default]
    SteepestAscent,
}

/// The HillClimb strategy is an iterative algorithm that starts with a single arbitrary solution
/// to a problem (unless the genotype seeds specific genes to sample a single starting point from),
/// then attempts to find a better solution by making an incremental change to the solution.
///
/// In the centralized module, only [HillClimbVariant::SteepestAscent] is supported, where
/// all neighbours are compared and the one with the best improvement is chosen.
///
/// The ending conditions are one or more of the following:
/// * target_fitness_score: when the ultimate goal in terms of fitness score is known and reached
/// * max_stale_generations: when the ultimate goal in terms of fitness score is unknown and one depends on some convergion
///   threshold, or one wants a duration limitation next to the target_fitness_score.
///   Set to a low value for [HillClimbVariant::SteepestAscent], preferably even `1`, unless
///   there is a replace_on_equal_fitness consideration or some remaining randomness in the neighbouring population
/// * max_generations: when the ultimate goal in terms of fitness score is unknown and there is a effort constraint
///
/// There are optional mutation distance limitations for
/// [RangeGenotype](crate::genotype::RangeGenotype) and
/// [MultiRangeGenotype](crate::genotype::MultiRangeGenotype) neighbouring chromosomes. Listed in
/// descending priority:
/// * With allele_mutation_scaled_range(s) set on genotype:
///     * Mutation distance only on edges of current scale (e.g. -1 and +1 for -1..-1 scale)
///     * Take both edges per gene for [HillClimbVariant::SteepestAscent]
///     * Scale down after max_stale_generations is reached and reset stale_generations to zero
///     * Only trigger max_stale_generations ending condition when already reached the smallest scale
///     * max_stale_generations could be set to 1, as there is no remaining randomness
/// * With allele_mutation_range(s) set on genotype:
///     * Mutation distance taken uniformly from mutation range
///     * Ensure to sample both a higher and lower value per gene for [HillClimbVariant::SteepestAscent]
///     * Standard max_stale_generations ending condition
///     * max_stale_generations should be set somewhat higher than 1 as there is some remaining randomness
/// * With only allele_range(s) set on genotype (not advised for hill climbing):
///     * Mutate uniformly over the complete allele range
///     * Ensure to sample both a higher and lower value per gene for [HillClimbVariant::SteepestAscent]
///     * Standard max_stale_generations ending condition
///     * max_stale_generations should be set somewhat higher than 1 as there is some remaining randomness
///
/// There are reporting hooks in the loop receiving the [HillClimbState], which can by handled by an
/// [StrategyReporter] (e.g. [HillClimbReporterDuration], [HillClimbReporterSimple]). But you are encouraged to
/// roll your own, see [StrategyReporter].
///
/// From the [HillClimbBuilder] level, there are several calling mechanisms:
/// * [call](HillClimbBuilder::call): this runs a single [HillClimb] strategy
/// * [call_repeatedly](HillClimbBuilder::call_repeatedly): this runs multiple independent [HillClimb]
///   strategies and returns the best one (or short circuits when the target_fitness_score is
///   reached)
/// * [call_par_repeatedly](HillClimbBuilder::call_par_repeatedly): this runs multiple independent
///   [HillClimb] strategies in parallel and returns the best one (or short circuits when the
///   target_fitness_score is reached).
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// See [HillClimbBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::centralized::strategy::hill_climb::prelude::*;
/// use genetic_algorithm::centralized::fitness::placeholders::SumStaticRange;
///
/// // the search space
/// let genotype = StaticRangeGenotype::<f32, 16, 33>::builder()     // f32 alleles
///     .with_genes_size(16)                    // 16 genes
///     .with_genes_hashing(true)               // store genes_hash on chromosome (required for fitness_cache and deduplication extension)
///     .with_allele_range(0.0..=1.0)           // allow gene values between 0.0 and 1.0
///     .with_allele_mutation_range(-0.1..=0.1) // neighbouring step size randomly sampled from range
///     .with_allele_mutation_scaled_range(vec![
///       -0.1..=0.1,
///       -0.01..=0.01,
///       -0.001..=0.001
///      ]) // neighbouring step size equal to start/end of each scaled range
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let hill_climb = HillClimb::builder()
///     .with_genotype(genotype)
///     .with_variant(HillClimbVariant::SteepestAscent)   // check all neighbours for each round
///     .with_fitness(SumStaticRange::new_with_precision(1e-5)) // sum the gene values of the chromosomes with precision 0.00001, which means multiply fitness score (isize) by 100_000
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the lowest sum
///     .with_fitness_cache(1000)                         // enable caching of fitness values (LRU size 1000), only works when genes_hash is stored in chromosome. Only useful for long stale runs
///     .with_target_fitness_score(0)                     // ending condition if sum of genes is <= 0.00001 in the best chromosome
///     .with_valid_fitness_score(100)                    // block ending conditions until at least the sum of genes <= 0.00100 is reached in the best chromosome
///     .with_max_stale_generations(1000)                 // stop searching if there is no improvement in fitness score for 1000 generations (per scaled_range)
///     .with_max_generations(1_000_000)                  // optional, stop searching after 1M generations
///     .with_replace_on_equal_fitness(true)              // optional, defaults to true, crucial for some type of problems with discrete fitness steps like nqueens
///     .with_reporter(HillClimbReporterSimple::new(100)) // optional, report every 100 generations
///     .with_rng_seed_from_u64(0)                        // for testing with deterministic results
///     .call()
///     .unwrap();
///
/// // it's all about the best genes after all
/// let (best_genes, best_fitness_score) = hill_climb.best_genes_and_fitness_score().unwrap();
/// assert_eq!(best_genes.into_iter().map(|v| v <= 1e-3).collect::<Vec<_>>(), vec![true; 16]);
/// assert_eq!(best_fitness_score, 0);
/// ```
pub struct HillClimb<
    G: HillClimbGenotype,
    F: Fitness<Genotype = G>,
    SR: StrategyReporter<Genotype = G>,
> {
    pub genotype: G,
    pub fitness: F,
    pub config: HillClimbConfig,
    pub state: HillClimbState<G>,
    pub reporter: SR,
    pub rng: SmallRng,
}

pub struct HillClimbConfig {
    pub variant: HillClimbVariant,
    pub fitness_ordering: FitnessOrdering,
    pub replace_on_equal_fitness: bool,

    pub target_fitness_score: Option<FitnessValue>,
    pub max_stale_generations: Option<usize>,
    pub max_generations: Option<usize>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub fitness_cache: Option<FitnessCache>,
}

/// Stores the state of the HillClimb strategy.
pub struct HillClimbState<G: HillClimbGenotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub stale_generations: usize,
    pub scale_generation: usize,
    pub best_generation: usize,
    pub best_fitness_score: Option<FitnessValue>,
    pub durations: HashMap<StrategyAction, Duration>,
    pub chromosome: Option<G::Chromosome>,
    pub population: Population<G::Chromosome>,
    pub current_scale_index: Option<usize>,
}

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>> Strategy<G>
    for HillClimb<G, F, SR>
{
    fn call(&mut self) {
        let now = Instant::now();
        self.reporter
            .on_enter(&self.genotype, &self.state, &self.config);

        self.setup();
        self.reporter
            .on_start(&self.genotype, &self.state, &self.config);
        while !self.is_finished() {
            self.state.increment_generation();
            self.genotype
                .load_best_genes(self.state.chromosome.as_mut().unwrap());
            self.genotype
                .chromosome_destructor_truncate(&mut self.state.population.chromosomes, 0);
            self.genotype.fill_neighbouring_population(
                self.state.chromosome.as_ref().unwrap(),
                &mut self.state.population,
                self.state.current_scale_index,
                &mut self.rng,
            );
            self.fitness.call_for_state_population(
                &self.genotype,
                &mut self.state,
                &self.config,
            );
            self.state.update_best_chromosome_from_state_population(
                &mut self.genotype,
                &self.config,
                &mut self.reporter,
                &mut self.rng,
            );
            self.reporter
                .on_new_generation(&self.genotype, &self.state, &self.config);
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

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>> HillClimb<G, F, StrategyReporterNoop<G>> {
    pub fn builder() -> HillClimbBuilder<G, F, StrategyReporterNoop<G>> {
        HillClimbBuilder::new()
    }
}
impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    HillClimb<G, F, SR>
{
    pub fn setup(&mut self) {
        let now = Instant::now();
        self.genotype.chromosomes_setup();

        let chromosome = self.genotype.chromosome_constructor_random(&mut self.rng);
        self.state.chromosome = Some(chromosome);
        self.genotype
            .save_best_genes(self.state.chromosome.as_ref().unwrap());
        self.state.best_generation = self.state.current_generation;
        self.state
            .add_duration(StrategyAction::SetupAndCleanup, now.elapsed());

        // init population with all seeds for first population if present, or just a single
        // random chromosome
        let population_size = self.genotype.seed_genes_list().len().max(1);
        self.state.population = self
            .genotype
            .population_constructor(population_size, &mut self.rng);

        self.fitness.call_for_state_population(
            &self.genotype,
            &mut self.state,
            &self.config,
        );
        self.state.update_best_chromosome_from_state_population(
            &mut self.genotype,
            &self.config,
            &mut self.reporter,
            &mut self.rng,
        );
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
        self.allow_finished_by_valid_fitness_score()
            && (self.is_finished_by_max_stale_generations()
                || self.is_finished_by_max_generations()
                || self.is_finished_by_target_fitness_score())
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.config.max_stale_generations {
            self.state.stale_generations >= max_stale_generations
        } else {
            false
        }
    }

    fn is_finished_by_max_generations(&self) -> bool {
        if let Some(max_generations) = self.config.max_generations {
            self.state.current_generation >= max_generations
        } else {
            false
        }
    }

    fn is_finished_by_target_fitness_score(&self) -> bool {
        if let Some(target_fitness_score) = self.config.target_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                match self.config.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= target_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= target_fitness_score,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn allow_finished_by_valid_fitness_score(&self) -> bool {
        if let Some(valid_fitness_score) = self.config.valid_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                match self.config.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= valid_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= valid_fitness_score,
                }
            } else {
                true
            }
        } else {
            true
        }
    }
}

impl StrategyConfig for HillClimbConfig {
    fn fitness_ordering(&self) -> FitnessOrdering {
        self.fitness_ordering
    }
    fn fitness_cache(&self) -> Option<&FitnessCache> {
        self.fitness_cache.as_ref()
    }
    fn replace_on_equal_fitness(&self) -> bool {
        self.replace_on_equal_fitness
    }
    fn variant(&self) -> StrategyVariant {
        StrategyVariant::HillClimb(self.variant)
    }
}

impl<G: HillClimbGenotype> StrategyState<G> for HillClimbState<G> {
    fn chromosome_as_ref(&self) -> &Option<G::Chromosome> {
        &self.chromosome
    }
    fn population_as_ref(&self) -> &Population<G::Chromosome> {
        &self.population
    }
    fn chromosome_as_mut(&mut self) -> &mut Option<G::Chromosome> {
        &mut self.chromosome
    }
    fn population_as_mut(&mut self) -> &mut Population<G::Chromosome> {
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

impl<G: HillClimbGenotype> HillClimbState<G> {
    fn update_best_chromosome_from_state_population<SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        config: &HillClimbConfig,
        reporter: &mut SR,
        rng: &mut SmallRng,
    ) {
        let now = Instant::now();
        if config.replace_on_equal_fitness {
            // shuffle, so we don't repeatedly take the same best chromosome in sideways move
            self.population.chromosomes.shuffle(rng);
        }
        if let Some(contending_chromosome) =
            self.population.best_chromosome(config.fitness_ordering)
        {
            match self.is_better_chromosome(
                contending_chromosome,
                &config.fitness_ordering,
                config.replace_on_equal_fitness,
            ) {
                (true, true) => {
                    self.best_generation = self.current_generation;
                    self.best_fitness_score = contending_chromosome.fitness_score();
                    genotype.save_best_genes(contending_chromosome);
                    reporter.on_new_best_chromosome(genotype, self, config);
                    self.reset_stale_generations();
                }
                (true, false) => {
                    genotype.save_best_genes(contending_chromosome);
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
    fn scale(&mut self, genotype: &G, config: &HillClimbConfig) {
        if let Some(current_scale_index) = self.current_scale_index {
            if let Some(max_stale_generations) = config.max_stale_generations {
                if let Some(max_scale_index) = genotype.max_scale_index() {
                    if self.stale_generations >= max_stale_generations
                        && current_scale_index < max_scale_index
                    {
                        self.current_scale_index = Some(current_scale_index + 1);
                        self.reset_scale_generation();
                        self.reset_stale_generations();
                    }
                }
            }
        }
    }
}

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    TryFrom<HillClimbBuilder<G, F, SR>> for HillClimb<G, F, SR>
{
    type Error = TryFromHillClimbBuilderError;

    fn try_from(builder: HillClimbBuilder<G, F, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires a HillClimbGenotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromHillClimbBuilderError("HillClimb requires a Fitness"))
        } else if builder.max_stale_generations.is_none()
            && builder.max_generations.is_none()
            && builder.target_fitness_score.is_none()
        {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires at least a max_stale_generations, max_generations or target_fitness_score ending condition",
            ))
        } else {
            let rng = builder.rng();
            let genotype = builder.genotype.unwrap();
            let state = HillClimbState::new(&genotype);

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),
                config: HillClimbConfig {
                    variant: builder.variant.unwrap_or_default(),
                    fitness_ordering: builder.fitness_ordering,
                    fitness_cache: builder.fitness_cache,
                    max_stale_generations: builder.max_stale_generations,
                    max_generations: builder.max_generations,
                    target_fitness_score: builder.target_fitness_score,
                    valid_fitness_score: builder.valid_fitness_score,
                    replace_on_equal_fitness: builder.replace_on_equal_fitness,
                },
                state,
                reporter: builder.reporter,
                rng,
            })
        }
    }
}

impl Default for HillClimbConfig {
    fn default() -> Self {
        Self {
            variant: Default::default(),
            fitness_ordering: FitnessOrdering::Maximize,
            fitness_cache: None,
            max_stale_generations: None,
            max_generations: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            replace_on_equal_fitness: false,
        }
    }
}
impl HillClimbConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: HillClimbGenotype> HillClimbState<G> {
    pub fn new(genotype: &G) -> Self {
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

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    fmt::Display for HillClimb<G, F, SR>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb:")?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;
        writeln!(f)?;

        writeln!(f, "{}", self.config)?;
        writeln!(f, "{}", self.state)?;
        writeln!(f, "{}", self.genotype)
    }
}

impl fmt::Display for HillClimbConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb_config:")?;
        writeln!(f, "  variant: {:?}", self.variant)?;

        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  max_generations: {:?}", self.max_generations)?;
        writeln!(f, "  valid_fitness_score: {:?}", self.valid_fitness_score)?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)
    }
}

impl<G: HillClimbGenotype> fmt::Display for HillClimbState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb_state:")?;
        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  stale generations: {:?}", self.stale_generations)?;
        writeln!(f, "  current scale index: {:?}", self.current_scale_index)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())
    }
}
