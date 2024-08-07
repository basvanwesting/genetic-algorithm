//! A solution strategy for finding the best chromosome, when search space is convex with little local optima or crossover is impossible or inefficient
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as HillClimbBuilder, TryFromBuilderError as TryFromHillClimbBuilderError,
};

use super::{Strategy, StrategyConfig, StrategyState};
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::IncrementalGenotype;
use crate::population::Population;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::fmt;
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

/// Defines the optional scaling of [ContinuousGenotype](crate::genotype::ContinuousGenotype) and
/// [MultiContinuousGenotype](crate::genotype::MultiContinuousGenotype) neighbouring_chromosomes
/// The base_scale is the starting scale which is steadily decreased to the min_scale by the scale
/// factor when no better chromosomes can be found in the current scale.
/// Only meaningful for SteepestAscent [variants](HillClimbVariant), as Stochastic variants are not
/// directed enough to benefit from scaling.
#[derive(Clone, Debug)]
pub struct Scaling {
    pub base_scale: f32,
    pub scale_factor: f32,
    pub min_scale: f32,
}

/// The HillClimb strategy is an iterative algorithm that starts with an arbitrary solution to a
/// problem, then attempts to find a better solution by making an incremental change to the
/// solution
///
/// There are 4 variants:
/// * [HillClimbVariant::Stochastic]: does not examine all neighbors before deciding how to move.
///   Rather, it selects a neighbor at random, and decides (based on the amount of improvement in
///   that neighbor) whether to move to that neighbor or to examine another
/// * [HillClimbVariant::SteepestAscent]: all neighbours are compared and the closest to the
///   solution is chosen
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
/// * min_scale: when the scaling drops below the precision and further refining is useless
///
/// The fitness is calculated each round:
/// * If the fitness is worse
///     * the mutation is ignored and the next round is started based on the current best chromosome
///     * if the scaling is set, the scale is reduced to zoom in on the local solution
///     * the stale generation counter is incremented (functionally)
/// * If the fitness is equal
///     * the mutated chromosome is taken for the next round.
///     * if the scaling is set, the scale is reset to its base scale
///     * the stale generation counter is incremented (functionally)
/// * If the fitness is better
///     * the mutated chromosome is taken for the next round.
///     * if the scaling is set, the scale is reset to its base scale
///     * the stale generation counter is reset (functionally)
///
/// There are reporting hooks in the loop receiving the [HillClimbState], which can by handled by an
/// [HillClimbReporter] (e.g. [HillClimbReporterNoop], [HillClimbReporterSimple]). But you are encouraged to
/// roll your own, see [HillClimbReporter].
///
/// See [HillClimbBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::hill_climb::prelude::*;
/// use genetic_algorithm::fitness::placeholders::SumContinuousGenotype;
///
/// // the search space
/// let genotype = ContinuousGenotype::builder() // f32 alleles
///     .with_genes_size(16)                     // 16 genes
///     .with_allele_range(0.0..1.0)             // values betwee 0.0 and 1.0
///     .with_allele_neighbour_range(-0.1..0.1)  // neighbouring step size or 0.1 in both directions
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let mut rng = rand::thread_rng(); // unused randomness provider implementing Trait rand::Rng
/// let hill_climb = HillClimb::builder()
///     .with_genotype(genotype)
///     .with_variant(HillClimbVariant::SteepestAscent)   // check all neighbours for each round
///     .with_fitness(SumContinuousGenotype(1e-5))        // sum the gene values of the chromosomes with precision 0.00001, which means multiply fitness score (isize) by 100_000
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the lowest sum
///     .with_multithreading(true)                        // use all cores for calculating the fitness of the neighbouring_population (only used with HillClimbVariant::SteepestAscent)
///     .with_scaling(Scaling::new(1.0, 0.8, 1e-5))       // start with neighbouring mutation scale 1.0 and multiply by 0.8 to zoom in on solution when stale, halt at 1e-5 scale
///     .with_target_fitness_score(10)                    // ending condition if sum of genes is <= 0.00010 in the best chromosome
///     .with_valid_fitness_score(100)                    // block ending conditions until at least the sum of genes <= 0.00100 is reached in the best chromosome
///     .with_max_stale_generations(1000)                 // stop searching if there is no improvement in fitness score for 1000 generations
///     .with_reporter(HillClimbReporterSimple::new(100)) // optional builder step, report every 100 generations
///     .call(&mut rng)
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = hill_climb.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes.into_iter().map(|v| v <= 1e-3).collect::<Vec<_>>(), vec![true; 16])
/// ```
pub struct HillClimb<
    G: IncrementalGenotype,
    F: Fitness<Genotype = G>,
    SR: HillClimbReporter<Genotype = G>,
> {
    genotype: G,
    fitness: F,
    pub config: HillClimbConfig,
    pub state: HillClimbState<G>,
    reporter: SR,
}

pub struct HillClimbConfig {
    pub variant: HillClimbVariant,
    pub fitness_ordering: FitnessOrdering,
    pub multithreading: bool,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub scaling: Option<Scaling>,
}

/// Stores the state of the HillClimb strategy. Next to the expected general fields, the following
/// strategy specific fields are added:
/// * current_scale: see [Scaling]
/// * contending_chromosome: available for all [variants](HillClimbVariant)
/// * neighbouring_population: only available for SteepestAscent [variants](HillClimbVariant)
pub struct HillClimbState<G: IncrementalGenotype> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<G>>,

    pub current_scale: Option<f32>,
    pub contending_chromosome: Option<Chromosome<G>>,
    pub neighbouring_population: Option<Population<G>>,
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>, SR: HillClimbReporter<Genotype = G>>
    Strategy<G> for HillClimb<G, F, SR>
{
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.state.reset_scaling(&self.config);

        let mut seed_chromosome = self.genotype.chromosome_factory(rng);
        self.fitness.call_for_chromosome(&mut seed_chromosome);
        self.state.set_best_chromosome(&seed_chromosome, true);

        let mut fitness_thread_local: Option<ThreadLocal<RefCell<F>>> = None;
        if self.config.multithreading {
            fitness_thread_local = Some(ThreadLocal::new());
        }

        self.reporter
            .on_start(&self.genotype, &self.state, &self.config);
        while !self.is_finished() {
            self.state.current_generation += 1;
            match self.config.variant {
                HillClimbVariant::Stochastic => {
                    let mut contending_chromosome = self.state.best_chromosome().unwrap();
                    self.genotype.mutate_chromosome_neighbour(
                        &mut contending_chromosome,
                        self.state.current_scale,
                        rng,
                    );
                    self.fitness.call_for_chromosome(&mut contending_chromosome);
                    self.state.update_best_chromosome_and_scale(
                        &contending_chromosome,
                        &self.config,
                        &mut self.reporter,
                    );
                    self.state.contending_chromosome = Some(contending_chromosome);
                }
                HillClimbVariant::StochasticSecondary => {
                    let mut contending_chromosome_primary = self.state.best_chromosome().unwrap();
                    self.genotype.mutate_chromosome_neighbour(
                        &mut contending_chromosome_primary,
                        self.state.current_scale,
                        rng,
                    );
                    self.fitness
                        .call_for_chromosome(&mut contending_chromosome_primary);
                    self.state.update_best_chromosome_and_scale(
                        &contending_chromosome_primary,
                        &self.config,
                        &mut self.reporter,
                    );

                    let mut contending_chromosome_secondary = contending_chromosome_primary.clone();
                    self.genotype.mutate_chromosome_neighbour(
                        &mut contending_chromosome_secondary,
                        self.state.current_scale,
                        rng,
                    );
                    self.fitness
                        .call_for_chromosome(&mut contending_chromosome_secondary);
                    self.state.update_best_chromosome_and_scale(
                        &contending_chromosome_secondary,
                        &self.config,
                        &mut self.reporter,
                    );
                    self.state.contending_chromosome = Some(contending_chromosome_secondary);
                }
                HillClimbVariant::SteepestAscent => {
                    let best_chromosome = self.state.best_chromosome_as_ref().unwrap();
                    let mut neighbouring_population = self
                        .genotype
                        .neighbouring_population(best_chromosome, self.state.current_scale);

                    self.fitness.call_for_population(
                        &mut neighbouring_population,
                        fitness_thread_local.as_ref(),
                    );

                    // shuffle, so we don't repeatedly take the same best chromosome in sideways move
                    neighbouring_population.chromosomes.shuffle(rng);
                    if let Some(contending_chromosome) =
                        neighbouring_population.best_chromosome(self.config.fitness_ordering)
                    {
                        self.state.update_best_chromosome_and_scale(
                            contending_chromosome,
                            &self.config,
                            &mut self.reporter,
                        );
                        self.state.contending_chromosome = Some(contending_chromosome.clone());
                    }
                    self.state.neighbouring_population = Some(neighbouring_population);
                }
                HillClimbVariant::SteepestAscentSecondary => {
                    let best_chromosome = self.state.best_chromosome_as_ref().unwrap();
                    let mut neighbouring_chromosomes = self
                        .genotype
                        .neighbouring_chromosomes(best_chromosome, self.state.current_scale);

                    neighbouring_chromosomes.append(
                        &mut neighbouring_chromosomes
                            .iter()
                            .flat_map(|chromosome| {
                                self.genotype
                                    .neighbouring_chromosomes(chromosome, self.state.current_scale)
                            })
                            .collect(),
                    );

                    let mut neighbouring_population = Population::new(neighbouring_chromosomes);

                    self.fitness.call_for_population(
                        &mut neighbouring_population,
                        fitness_thread_local.as_ref(),
                    );

                    // shuffle, so we don't repeatedly take the same best chromosome in sideways move
                    neighbouring_population.chromosomes.shuffle(rng);
                    if let Some(contending_chromosome) =
                        neighbouring_population.best_chromosome(self.config.fitness_ordering)
                    {
                        self.state.update_best_chromosome_and_scale(
                            contending_chromosome,
                            &self.config,
                            &mut self.reporter,
                        );
                        self.state.contending_chromosome = Some(contending_chromosome.clone());
                    }
                    self.state.neighbouring_population = Some(neighbouring_population);
                }
            }
            self.reporter.on_new_generation(&self.state, &self.config);
        }
        self.reporter.on_finish(&self.state, &self.config);
    }
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.state.best_chromosome()
    }
    fn best_generation(&self) -> usize {
        self.state.best_generation
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.state.best_fitness_score()
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
    fn is_finished(&self) -> bool {
        self.allow_finished_by_valid_fitness_score()
            && (self.is_finished_by_max_stale_generations()
                || self.is_finished_by_target_fitness_score()
                || self.is_finished_by_min_scale())
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.config.max_stale_generations {
            self.state.current_generation - self.state.best_generation >= max_stale_generations
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

    fn is_finished_by_min_scale(&self) -> bool {
        if let Some(current_scale) = self.state.current_scale {
            current_scale < self.config.scaling.as_ref().unwrap().min_scale
        } else {
            false
        }
    }
}

impl StrategyConfig for HillClimbConfig {
    fn fitness_ordering(&self) -> FitnessOrdering {
        self.fitness_ordering
    }
    fn multithreading(&self) -> bool {
        self.multithreading
    }
}

impl<G: IncrementalGenotype> StrategyState<G> for HillClimbState<G> {
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.best_chromosome.clone()
    }
    fn best_chromosome_as_ref(&self) -> Option<&Chromosome<G>> {
        self.best_chromosome.as_ref()
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
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
    fn set_best_chromosome(
        &mut self,
        best_chromosome: &Chromosome<G>,
        improved_fitness: bool,
    ) -> (bool, bool) {
        self.best_chromosome = Some(best_chromosome.clone());
        if improved_fitness {
            self.best_generation = self.current_generation;
        }
        (true, improved_fitness)
    }
}

impl<G: IncrementalGenotype> HillClimbState<G> {
    fn update_best_chromosome_and_scale<SR: HillClimbReporter<Genotype = G>>(
        &mut self,
        contending_chromosome: &Chromosome<G>,
        config: &HillClimbConfig,
        reporter: &mut SR,
    ) {
        match self.update_best_chromosome(contending_chromosome, &config.fitness_ordering, true) {
            (true, true) => {
                reporter.on_new_best_chromosome(self, config);
                self.reset_scaling(config);
            }
            (true, false) => {
                reporter.on_new_best_chromosome_equal_fitness(self, config);
                self.reset_scaling(config);
            }
            _ => self.scale_down(config),
        }
    }

    fn reset_scaling(&mut self, hill_climb_config: &HillClimbConfig) {
        self.current_scale = hill_climb_config.scaling.as_ref().map(|s| s.base_scale);
    }

    fn scale_down(&mut self, hill_climb_config: &HillClimbConfig) {
        if let Some(current_scale) = self.current_scale {
            self.current_scale =
                Some(current_scale * hill_climb_config.scaling.as_ref().unwrap().scale_factor);
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
        } else if builder.max_stale_generations.is_none()
            && builder.target_fitness_score.is_none()
            && builder.scaling.is_none()
        {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires at least a max_stale_generations, target_fitness_score or scaling ending condition",
            ))
        } else {
            Ok(Self {
                genotype: builder.genotype.unwrap(),
                fitness: builder.fitness.unwrap(),
                config: HillClimbConfig {
                    variant: builder.variant.unwrap_or(HillClimbVariant::Stochastic),
                    fitness_ordering: builder.fitness_ordering,
                    multithreading: builder.multithreading,
                    max_stale_generations: builder.max_stale_generations,
                    target_fitness_score: builder.target_fitness_score,
                    valid_fitness_score: builder.valid_fitness_score,
                    scaling: builder.scaling,
                },
                state: HillClimbState::new(),
                reporter: builder.reporter,
            })
        }
    }
}

impl Default for HillClimbConfig {
    fn default() -> Self {
        Self {
            variant: HillClimbVariant::default(),
            fitness_ordering: FitnessOrdering::Maximize,
            multithreading: false,
            max_stale_generations: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            scaling: None,
        }
    }
}
impl HillClimbConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: IncrementalGenotype> Default for HillClimbState<G> {
    fn default() -> Self {
        Self {
            current_iteration: 0,
            current_generation: 0,
            current_scale: None,
            best_generation: 0,
            best_chromosome: None,
            contending_chromosome: None,
            neighbouring_population: None,
        }
    }
}
impl<G: IncrementalGenotype> HillClimbState<G> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scaling {
    pub fn new(base_scale: f32, scale_factor: f32, min_scale: f32) -> Self {
        Self {
            base_scale,
            scale_factor,
            min_scale,
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
        writeln!(f, "  multithreading: {:?}", self.multithreading)?;
        writeln!(f, "  scaling: {:?}", self.scaling)
    }
}

impl<G: IncrementalGenotype> fmt::Display for HillClimbState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb_state:")?;
        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  current scale: {:?}", self.current_scale)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
