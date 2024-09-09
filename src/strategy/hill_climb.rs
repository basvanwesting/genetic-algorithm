//! A solution strategy for finding the best chromosome, when search space is convex with little local optima or crossover is impossible or inefficient
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as HillClimbBuilder, TryFromBuilderError as TryFromHillClimbBuilderError,
};

use super::{Strategy, StrategyAction, StrategyConfig, StrategyState};
use crate::chromosome::{Chromosome, OwnesGenes};
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::IncrementalGenotype;
use crate::population::Population;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use thread_local::ThreadLocal;

pub use self::reporter::Log as HillClimbReporterLog;
pub use self::reporter::Noop as HillClimbReporterNoop;
pub use self::reporter::Reporter as HillClimbReporter;
pub use self::reporter::Simple as HillClimbReporterSimple;

#[derive(Clone, Debug, Default)]
pub enum HillClimbVariant {
    #[default]
    Stochastic,
    StochasticSecondary,
    SteepestAscent,
    SteepestAscentSecondary,
}

/// The HillClimb strategy is an iterative algorithm that starts with an arbitrary solution to a
/// problem, then attempts to find a better solution by making an incremental change to the
/// solution
///
/// There are 4 variants:
/// * [HillClimbVariant::Stochastic]: does not examine all neighbors before deciding how to move.
///   Rather, it selects a neighbor at random, and decides (based on the improvement in that
///   neighbour) whether to move to that neighbor or to examine another
/// * [HillClimbVariant::SteepestAscent]: all neighbours are compared and the one with the best
///   improvement is chosen.
/// * [HillClimbVariant::StochasticSecondary]: like Stochastic, but also randomly tries a random
///   neighbour of the neighbour. Useful when a single mutation would generally not lead to
///   improvement, because the problem space behaves more like a
///   [UniqueGenotype](crate::genotype::UniqueGenotype) where genes must be swapped (but the
///   UniqueGenotype doesn't map to the problem space well)
/// * [HillClimbVariant::SteepestAscentSecondary]: like SteepestAscent, but also neighbours of
///   neighbours are in scope. This is O(n^2) with regards to the SteepestAscent variant, so use
///   with caution.
///
/// The ending conditions are one or more of the following:
/// * target_fitness_score: when the ultimate goal in terms of fitness score is known and reached
/// * max_stale_generations: when the ultimate goal in terms of fitness score is unknown and one depends on some convergion
///   threshold, or one wants a duration limitation next to the target_fitness_score
///
/// There are optional mutation distance limitations for
/// [RangeGenotype](crate::genotype::RangeGenotype) and
/// [MultiRangeGenotype](crate::genotype::MultiRangeGenotype) neighbouring chromosomes. Listed in
/// descending priority:
/// * With allele_mutation_scaled_range(s) set on genotype:
///     * Mutation distance only on edges of current scale (e.g. -1 and +1 for -1..-1 scale)
///         * Pick random edge for [HillClimbVariant::Stochastic]
///         * Take both edges per gene for [HillClimbVariant::SteepestAscent]
///     * Scale down after max_stale_generations is reached and reset stale_generations to zero
///     * Only trigger max_stale_generations ending condition when already reached the smallest scale
/// * With allele_mutation_range(s) set on genotype:
///     * Mutation distance taken uniformly from mutation range
///         * Sample single random value for [HillClimbVariant::Stochastic]
///         * Ensure to sample both a higer and lower value per gene for [HillClimbVariant::SteepestAscent]
///     * Standard max_stale_generations ending condition
/// * With only allele_range(s) set on genotype:
///     * Mutate uniformly over the complete allele range
///         * Sample single random value for [HillClimbVariant::Stochastic]
///         * Not valid for [HillClimbVariant::SteepestAscent]
///     * Standard max_stale_generations ending condition
///
/// Using scaling for [HillClimbVariant::StochasticSecondary] and
/// [HillClimbVariant::SteepestAscentSecondary] doesn't make sense, though it will work.
///
/// There are reporting hooks in the loop receiving the [HillClimbState], which can by handled by an
/// [HillClimbReporter] (e.g. [HillClimbReporterNoop], [HillClimbReporterSimple]). But you are encouraged to
/// roll your own, see [HillClimbReporter].
///
/// From the [HillClimbBuilder] level, there are several calling mechanisms:
/// * [call](HillClimbBuilder::call): this runs a single [HillClimb] strategy
/// * [call_repeatedly](HillClimbBuilder::call_repeatedly): this runs multiple independent [HillClimb]
///   strategies and returns the best one (or short circuits when the target_fitness_score is
///   reached)
/// * [call_par_repeatedly](HillClimbBuilder::call_par_repeatedly): this runs multiple independent
///   [HillClimb] strategies in parallel and returns the best one (or short circuits when the
///   target_fitness_score is reached). This is separate and independent from the
///   `with_par_fitness()` flag on the builder, which determines multithreading of the fitness
///   calculation inside the [HillClimb] strategy. Both can be combined.
///
/// Multithreading inside the [HillClimbVariant::Stochastic] and
/// [HillClimbVariant::StochasticSecondary] using the `with_par_fitness()` builder step does
/// nothing, due to the sequential nature of the search. But
/// [call_par_repeatedly](HillClimbBuilder::call_par_repeatedly) still effectively multithreads for
/// these variants as the sequential nature is only internal to the [HillClimb] strategy.
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// See [HillClimbBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::hill_climb::prelude::*;
/// use genetic_algorithm::fitness::placeholders::SumGenes;
///
/// // the search space
/// let genotype = RangeGenotype::builder()     // f32 alleles
///     .with_genes_size(16)                    // 16 genes
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
///     .with_fitness(SumGenes::new_with_precision(1e-5)) // sum the gene values of the chromosomes with precision 0.00001, which means multiply fitness score (isize) by 100_000
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the lowest sum
///     .with_par_fitness(true)                           // optional, defaults to false, use parallel fitness calculation
///     .with_target_fitness_score(10)                    // ending condition if sum of genes is <= 0.00010 in the best chromosome
///     .with_valid_fitness_score(100)                    // block ending conditions until at least the sum of genes <= 0.00100 is reached in the best chromosome
///     .with_max_stale_generations(1000)                 // stop searching if there is no improvement in fitness score for 1000 generations
///     .with_replace_on_equal_fitness(true)              // optional, defaults to true, crucial for some type of problems with discrete fitness steps like nqueens
///     .with_reporter(HillClimbReporterSimple::new(100)) // optional, report every 100 generations
///     .with_rng_seed_from_u64(0)                        // for testing with deterministic results
///     .call()
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_genes = hill_climb.best_genes().unwrap();
/// assert_eq!(best_genes.into_iter().map(|v| v <= 1e-3).collect::<Vec<_>>(), vec![true; 16])
/// ```
pub struct HillClimb<
    G: IncrementalGenotype,
    F: Fitness<Genotype = G>,
    SR: HillClimbReporter<Genotype = G>,
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
    pub par_fitness: bool,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub replace_on_equal_fitness: bool,
}

/// Stores the state of the HillClimb strategy. Next to the expected general fields, the following
/// strategy specific fields are added:
/// * current_scale_index: current index of [IncrementalGenotype]'s allele_mutation_scaled_range
/// * max_scale_index: max index of [IncrementalGenotype]'s allele_mutation_scaled_range
/// * contending_chromosome: available for all [variants](HillClimbVariant)
/// * neighbouring_population: only available for SteepestAscent [variants](HillClimbVariant)
pub struct HillClimbState<G: IncrementalGenotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub stale_generations: usize,
    pub best_generation: usize,
    pub best_fitness_score: Option<FitnessValue>,
    pub durations: HashMap<StrategyAction, Duration>,

    pub current_scale_index: Option<usize>,
    pub max_scale_index: usize,
    pub chromosome: G::Chromosome,
    pub population: Population<G::Chromosome>,
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>, SR: HillClimbReporter<Genotype = G>>
    Strategy<G> for HillClimb<G, F, SR>
{
    fn call(&mut self) {
        let now = Instant::now();
        let mut fitness_thread_local: Option<ThreadLocal<RefCell<F>>> = None;
        if self.config.par_fitness {
            fitness_thread_local = Some(ThreadLocal::new());
        }

        self.init();
        self.reporter.on_start(&self.state, &self.config);
        while !self.is_finished() {
            self.state.current_generation += 1;
            match self.config.variant {
                HillClimbVariant::Stochastic => {
                    let now_data = Instant::now();
                    self.genotype.load_best_genes(&mut self.state.chromosome);
                    self.state.add_duration(
                        StrategyAction::ChromosomeDataDropAndCopy,
                        now_data.elapsed(),
                    );

                    self.genotype.mutate_chromosome_genes(
                        1,
                        true,
                        &mut self.state.chromosome,
                        self.state.current_scale_index,
                        &mut self.rng,
                    );
                    self.fitness
                        .call_for_state_chromosome(&mut self.state, &self.genotype);
                    self.state.update_best_chromosome_from_state_chromosome(
                        &mut self.genotype,
                        &self.config,
                        &mut self.reporter,
                    );
                }
                HillClimbVariant::StochasticSecondary => {
                    let now_data = Instant::now();
                    self.genotype.load_best_genes(&mut self.state.chromosome);
                    self.state.add_duration(
                        StrategyAction::ChromosomeDataDropAndCopy,
                        now_data.elapsed(),
                    );

                    self.genotype.mutate_chromosome_genes(
                        1,
                        true,
                        &mut self.state.chromosome,
                        self.state.current_scale_index,
                        &mut self.rng,
                    );
                    self.fitness
                        .call_for_state_chromosome(&mut self.state, &self.genotype);

                    self.state.update_best_chromosome_from_state_chromosome(
                        &mut self.genotype,
                        &self.config,
                        &mut self.reporter,
                    );

                    // second round
                    self.genotype.mutate_chromosome_genes(
                        1,
                        true,
                        &mut self.state.chromosome,
                        self.state.current_scale_index,
                        &mut self.rng,
                    );
                    self.fitness
                        .call_for_state_chromosome(&mut self.state, &self.genotype);
                    self.state.update_best_chromosome_from_state_chromosome(
                        &mut self.genotype,
                        &self.config,
                        &mut self.reporter,
                    );
                }
                HillClimbVariant::SteepestAscent => {
                    self.genotype.load_best_genes(&mut self.state.chromosome);
                    self.state.population = self.genotype.neighbouring_population(
                        &self.state.chromosome,
                        self.state.current_scale_index,
                        &mut self.rng,
                    );
                    self.fitness.call_for_state_population(
                        &mut self.state,
                        &mut self.genotype,
                        fitness_thread_local.as_ref(),
                    );
                    self.state.update_best_chromosome_from_state_population(
                        &mut self.genotype,
                        &self.config,
                        &mut self.reporter,
                        &mut self.rng,
                    );
                }
                HillClimbVariant::SteepestAscentSecondary => {
                    self.genotype.load_best_genes(&mut self.state.chromosome);
                    let mut neighbouring_chromosomes = self.genotype.neighbouring_chromosomes(
                        &self.state.chromosome,
                        self.state.current_scale_index,
                        &mut self.rng,
                    );
                    neighbouring_chromosomes.append(
                        &mut neighbouring_chromosomes
                            .iter()
                            .flat_map(|chromosome| {
                                self.genotype.neighbouring_chromosomes(
                                    chromosome,
                                    self.state.current_scale_index,
                                    &mut self.rng,
                                )
                            })
                            .collect(),
                    );
                    self.state.population = Population::new(neighbouring_chromosomes);
                    self.fitness.call_for_state_population(
                        &mut self.state,
                        &mut self.genotype,
                        fitness_thread_local.as_ref(),
                    );
                    self.state.update_best_chromosome_from_state_population(
                        &mut self.genotype,
                        &self.config,
                        &mut self.reporter,
                        &mut self.rng,
                    );
                }
            }
            self.reporter.on_new_generation(&self.state, &self.config);
            self.state.scale(&self.config);
        }
        self.state.close_duration(now.elapsed());
        self.reporter.on_finish(&self.state, &self.config);
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
}
impl<G: IncrementalGenotype, F: Fitness<Genotype = G>, SR: HillClimbReporter<Genotype = G>>
    HillClimb<G, F, SR>
where
    G::Chromosome: OwnesGenes<Genes = G::Genes>,
{
    pub fn best_chromosome(&self) -> Option<G::Chromosome> {
        if let Some(best_genes) = self.best_genes() {
            let mut chromosome = G::Chromosome::new(best_genes);
            chromosome.set_fitness_score(self.best_fitness_score());
            Some(chromosome)
        } else {
            None
        }
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> HillClimb<G, F, HillClimbReporterNoop<G>> {
    pub fn builder() -> HillClimbBuilder<G, F, HillClimbReporterNoop<G>> {
        HillClimbBuilder::new()
    }
}
impl<G: IncrementalGenotype, F: Fitness<Genotype = G>, SR: HillClimbReporter<Genotype = G>>
    HillClimb<G, F, SR>
{
    pub fn init(&mut self) {
        let now = Instant::now();
        self.reporter
            .on_init(&self.genotype, &self.state, &self.config);

        self.genotype.chromosomes_init();
        self.state.chromosome = self.genotype.chromosome_constructor(&mut self.rng);
        self.state.add_duration(StrategyAction::Init, now.elapsed());

        self.fitness
            .call_for_state_chromosome(&mut self.state, &self.genotype);

        // best by definition, remove the chromosome placeholder
        self.state.best_generation = self.state.current_generation;
        self.state.best_fitness_score = self.state.chromosome.fitness_score();
        self.genotype.save_best_genes(&self.state.chromosome);

        self.reporter
            .on_new_best_chromosome(&self.state, &self.config);
    }
    fn is_finished(&self) -> bool {
        self.allow_finished_by_valid_fitness_score()
            && (self.is_finished_by_max_stale_generations()
                || self.is_finished_by_target_fitness_score())
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.config.max_stale_generations {
            self.state.stale_generations >= max_stale_generations
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
    fn par_fitness(&self) -> bool {
        self.par_fitness
    }
    fn replace_on_equal_fitness(&self) -> bool {
        self.replace_on_equal_fitness
    }
}

impl<G: IncrementalGenotype> StrategyState<G> for HillClimbState<G> {
    fn chromosome_as_ref(&self) -> &G::Chromosome {
        &self.chromosome
    }
    fn chromosome_as_mut(&mut self) -> &mut G::Chromosome {
        &mut self.chromosome
    }
    fn population_as_ref(&self) -> &Population<G::Chromosome> {
        &self.population
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
    fn stale_generations(&self) -> usize {
        self.stale_generations
    }
    fn increment_stale_generations(&mut self) {
        self.stale_generations += 1;
    }
    fn reset_stale_generations(&mut self) {
        self.stale_generations = 0;
    }
    fn add_duration(&mut self, action: StrategyAction, duration: Duration) {
        *self.durations.entry(action).or_default() += duration;
    }
    fn total_duration(&self) -> Duration {
        self.durations.values().sum()
    }
}

impl<G: IncrementalGenotype> HillClimbState<G> {
    fn update_best_chromosome_from_state_chromosome<SR: HillClimbReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        config: &HillClimbConfig,
        reporter: &mut SR,
    ) {
        let now = Instant::now();
        match self.is_better_chromosome(
            &self.chromosome,
            &config.fitness_ordering,
            config.replace_on_equal_fitness,
        ) {
            (true, true) => {
                self.best_generation = self.current_generation;
                self.best_fitness_score = self.chromosome.fitness_score();
                genotype.save_best_genes(&self.chromosome);
                reporter.on_new_best_chromosome(self, config);
                self.reset_stale_generations();
            }
            (true, false) => {
                genotype.save_best_genes(&self.chromosome);
                reporter.on_new_best_chromosome_equal_fitness(self, config);
                self.increment_stale_generations()
            }
            _ => self.increment_stale_generations(),
        }
        self.add_duration(StrategyAction::UpdateBestChromosome, now.elapsed());
    }
    fn update_best_chromosome_from_state_population<SR: HillClimbReporter<Genotype = G>>(
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
                    reporter.on_new_best_chromosome(self, config);
                    self.reset_stale_generations();
                }
                (true, false) => {
                    genotype.save_best_genes(contending_chromosome);
                    reporter.on_new_best_chromosome_equal_fitness(self, config);
                    self.increment_stale_generations()
                }
                _ => self.increment_stale_generations(),
            }
        } else {
            self.increment_stale_generations();
        }
        self.add_duration(StrategyAction::UpdateBestChromosome, now.elapsed());
    }
    fn scale(&mut self, config: &HillClimbConfig) {
        if let Some(current_scale_index) = self.current_scale_index {
            if let Some(max_stale_generations) = config.max_stale_generations {
                if self.stale_generations >= max_stale_generations
                    && current_scale_index < self.max_scale_index
                {
                    self.current_scale_index = Some(current_scale_index + 1);
                    self.reset_stale_generations();
                }
            }
        }
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>, SR: HillClimbReporter<Genotype = G>>
    TryFrom<HillClimbBuilder<G, F, SR>> for HillClimb<G, F, SR>
{
    type Error = TryFromHillClimbBuilderError;

    fn try_from(builder: HillClimbBuilder<G, F, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires a Genotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromHillClimbBuilderError("HillClimb requires a Fitness"))
        } else if builder.max_stale_generations.is_none() && builder.target_fitness_score.is_none()
        {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires at least a max_stale_generations or target_fitness_score ending condition",
            ))
        } else {
            let rng = builder.rng();
            let genotype = builder.genotype.unwrap();
            let state = HillClimbState::new(&genotype);

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),
                config: HillClimbConfig {
                    variant: builder.variant.unwrap_or(HillClimbVariant::Stochastic),
                    fitness_ordering: builder.fitness_ordering,
                    par_fitness: builder.par_fitness,
                    max_stale_generations: builder.max_stale_generations,
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
            variant: HillClimbVariant::default(),
            fitness_ordering: FitnessOrdering::Maximize,
            par_fitness: false,
            max_stale_generations: None,
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

impl<G: IncrementalGenotype> HillClimbState<G> {
    pub fn new(genotype: &G) -> Self {
        let base = Self {
            current_iteration: 0,
            current_generation: 0,
            stale_generations: 0,
            current_scale_index: None,
            max_scale_index: 0,
            best_generation: 0,
            best_fitness_score: None,
            chromosome: genotype.chromosome_constructor_empty(), //invalid, temporary
            population: Population::new_empty(),
            durations: HashMap::new(),
        };
        if let Some(max_scale_index) = genotype.max_scale_index() {
            Self {
                current_scale_index: Some(0),
                max_scale_index,
                ..base
            }
        } else {
            base
        }
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>, SR: HillClimbReporter<Genotype = G>>
    fmt::Display for HillClimb<G, F, SR>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(f, "{}", self.config)?;
        writeln!(f, "{}", self.state)
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
        writeln!(f, "  valid_fitness_score: {:?}", self.valid_fitness_score)?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  par_fitness: {:?}", self.par_fitness)
    }
}

impl<G: IncrementalGenotype> fmt::Display for HillClimbState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb_state:")?;
        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  stale generations: {:?}", self.stale_generations)?;
        writeln!(
            f,
            "  scale index (current/max): {:?}/{}",
            self.current_scale_index, self.max_scale_index
        )?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())
    }
}
