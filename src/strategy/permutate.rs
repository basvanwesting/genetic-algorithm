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
use crate::chromosome::{Chromosome, Genes};
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::PermutateGenotype;
use crate::population::Population;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::sync_channel;
use std::sync::Arc;
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
/// [MultiRangeGenotype](crate::genotype::MultiRangeGenotype) chromosomes, see [crate::genotype::MutationType].
/// * With MutationType::Step
///     * Traverses the whole allele_range(s) with the step size.
/// * With MutationType::StepScaled
///     * First scale (index = 0) traverses the whole allele_range(s) with the first scale as step size.
///     * Other scales (index > 0) center around the best chromomsome of the previous scale,
///     traversing the previous scale step as bounds around the best chromosome with the
///     current scale as inner step size.
///     * Scale down after grid is fully traversed
/// * With MutationType::Discrete
///     * Always permutate all values, just like ListGenotype
///
/// There are reporting hooks in the loop receiving the [PermutateState], which can by handled by an
/// [StrategyReporter] (e.g. [PermutateReporterDuration], [PermutateReporterSimple]). But you are encouraged to
/// roll your own, see [StrategyReporter].
///
/// Below is the exact order of actions and hooks
/// * [reporter](crate::strategy::reporter) on_enter hook
/// * setup
/// * [reporter](crate::strategy::reporter) on_start hook
/// * loop while not finished
///   * (parallel) iterate over chromosomes
///     * increment generation
///     * [fitness](crate::fitness) calculation
///     * update best chromosome
///     * [reporter](crate::strategy::reporter) on_generation_complete hook
///   * scale and reset ending conditions for new scale
///   * check ending conditions
/// * [reporter](crate::strategy::reporter) on_finish hook
/// * cleanup
/// * [reporter](crate::strategy::reporter) on_exit hook
///
/// See [PermutateBuilder] for initialization options.
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::permutate::prelude::*;
/// use genetic_algorithm::fitness::placeholders::CountTrue;
///
/// // the search space
/// let genotype = BinaryGenotype::builder() // boolean alleles
///     .with_genes_size(12)                 // 12 genes per chromosome
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let permutate = Permutate::builder()
///     .with_genotype(genotype)
///     .with_fitness(CountTrue)                          // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_par_fitness(true)                           // optional, defaults to false, use parallel fitness calculation
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
    pub state: PermutateState<G>,
    pub reporter: SR,
}

pub struct PermutateConfig {
    pub variant: PermutateVariant,
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
    // optional ending condition: stop permutating as soon as the best chromosome reaches this
    // fitness score (instead of exhausting the whole permutation space). Checked once per
    // chromosome.
    pub target_fitness_score: Option<FitnessValue>,
    // cooperative abort signal, checked once per chromosome. When set the run stops early,
    // returning the best chromosome found so far.
    pub abort_flag: Option<Arc<AtomicBool>>,
}

/// Stores the state of the Permutate strategy
pub struct PermutateState<G: PermutateGenotype> {
    pub current_iteration: usize,
    // generation counters are usize, not BigUint: they count permutations actually evaluated, which
    // is bounded by wall-clock time and stays well within usize (overflowing u64 needs ~1.8e19
    // evaluations — centuries even at 1e9/s), so a terminating run never overflows them. Only the
    // permutation space *size* (chromosome_permutations_size) is BigUint, as it is computed
    // analytically and need not be enumerated.
    pub current_generation: usize,
    pub stale_generations: usize,
    pub scale_generation: usize,
    pub best_generation: usize,
    pub best_fitness_score: Option<FitnessValue>,
    pub best_chromosome: Option<Chromosome<G::Allele>>,
    pub chromosome: Option<Chromosome<G::Allele>>,
    pub population: Population<G::Allele>,
    pub durations: HashMap<StrategyAction, Duration>,
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
            if self.config.par_fitness {
                self.call_parallel()
            } else {
                self.call_sequential()
            }
            self.state.scale(&mut self.genotype, &self.config);
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
    fn best_genes(&self) -> Option<Genes<G::Allele>> {
        self.state
            .best_chromosome
            .as_ref()
            .map(|c| c.genes().clone())
    }
    fn flush_reporter(&mut self, output: &mut Vec<u8>) {
        self.reporter.flush(output);
    }
}
impl<G: PermutateGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    Permutate<G, F, SR>
{
    pub fn best_chromosome(&self) -> Option<Chromosome<G::Allele>> {
        if let Some(best_genes) = self.best_genes() {
            let mut chromosome = Chromosome::<G::Allele>::new(best_genes);
            chromosome.set_fitness_score(self.best_fitness_score());
            Some(chromosome)
        } else {
            None
        }
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
        self.state.chromosome = self.genotype.chromosome_permutations_into_iter(None).next();
        self.state
            .add_duration(StrategyAction::SetupAndCleanup, now.elapsed());
        self.fitness
            .call_for_state_chromosome(&self.genotype, &mut self.state, &self.config);
        self.state.update_best_chromosome_and_report(
            &self.genotype,
            &self.config,
            &mut self.reporter,
        );

        if self.state.best_fitness_score().is_none() {
            self.state.best_generation = self.state.current_generation;
            self.state
                .best_chromosome
                .clone_from(&self.state.chromosome);
            self.reporter
                .on_new_best_chromosome(&self.genotype, &self.state, &self.config);
            self.state.reset_stale_generations();
        }
    }
    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.state.chromosome.take();
        self.state.population.chromosomes.clear();
        self.state
            .add_duration(StrategyAction::SetupAndCleanup, now.elapsed());
    }
    // Only evaluated between sweeps, by the call() loop condition: a full call_sequential /
    // call_parallel sweep runs first, then scale(), then this check. The inner per-chromosome loops
    // use is_conclusive() directly, never is_finished(). is_exhausted() relies on this timing.
    fn is_finished(&self) -> bool {
        self.is_conclusive() || self.is_exhausted()
    }
    // conclusive end of the run: the target fitness score was reached, or the run was aborted.
    // Honoured per chromosome inside the inner permutation loops (before scale exhaustion is
    // meaningful), as opposed to is_finished which also ends on scale exhaustion.
    fn is_conclusive(&self) -> bool {
        // cooperative abort
        if self
            .config
            .abort_flag
            .as_ref()
            .is_some_and(|flag| flag.load(Ordering::Relaxed))
        {
            return true;
        }
        // target_fitness_score
        if let Some(target_fitness_score) = self.config.target_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                return match self.config.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= target_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= target_fitness_score,
                };
            }
        }
        false
    }
    // exhausted end of the run: the permutation space has been fully enumerated at the finest
    // scale, with no finer scale left to advance to.
    //
    // Unlike HillClimb/Evolve there is no max_generations budget to compare against — a scale ends
    // only when its sweep completes (the permutation iterator runs dry), and the space size is a
    // BigUint, not a usize generation count. So the signal is indirect: `scale_generation > 0`.
    //
    // This works *only because is_exhausted is evaluated between sweeps* (see is_finished). Within
    // a sweep scale_generation is incremented per chromosome and is almost always > 0 — it does
    // NOT mean "exhausted" there. At the outer-loop boundary, however, scale() has just run and
    // resets scale_generation to 0 whenever it advanced to a finer scale; so a non-zero value can
    // only mean the just-completed sweep was at the finest scale (scale() could not advance).
    // Before the first sweep it is also 0, so the run is not reported exhausted prematurely.
    fn is_exhausted(&self) -> bool {
        self.state.scale_generation > 0
    }

    fn call_sequential(&mut self) {
        for chromosome in self
            .genotype
            .clone()
            .chromosome_permutations_into_iter(self.state.best_chromosome.as_ref())
        {
            self.state.increment_generation();
            self.state.chromosome.replace(chromosome);
            self.fitness
                .call_for_state_chromosome(&self.genotype, &mut self.state, &self.config);
            self.state.update_best_chromosome_and_report(
                &self.genotype,
                &self.config,
                &mut self.reporter,
            );
            self.reporter
                .on_generation_complete(&self.genotype, &self.state, &self.config);
            if self.is_conclusive() {
                break;
            }
        }
    }
    fn call_parallel(&mut self) {
        // A plain thread scope, not rayon::scope: rayon::scope runs this body on a rayon worker,
        // which the receiver below then blocks while the producer needs a rayon worker to run
        // (deadlock with a single rayon thread)
        std::thread::scope(|s| {
            let thread_genotype = self.genotype.clone();
            let thread_best_chromosome = self.state.best_chromosome.clone();
            let fitness = self.fitness.clone();
            let fitness_cache = self.config.fitness_cache();
            let (sender, receiver) = sync_channel(1000);

            // Internal stop signal, set by the consumer below once the target fitness score is
            // reached. Combined with the external abort_flag, it lets the producer stop feeding
            // permutations early instead of exhausting the whole space.
            let stop = Arc::new(AtomicBool::new(false));
            let producer_stop = Arc::clone(&stop);
            let abort_flag = self.config.abort_flag.clone();

            s.spawn(move || {
                thread_genotype
                    .chromosome_permutations_into_iter(thread_best_chromosome.as_ref())
                    .take_while(|_| {
                        !producer_stop.load(Ordering::Relaxed)
                            && !abort_flag
                                .as_ref()
                                .is_some_and(|flag| flag.load(Ordering::Relaxed))
                    })
                    .par_bridge()
                    .for_each_with((sender, fitness), |(sender, fitness), mut chromosome| {
                        let now = Instant::now();
                        fitness.call_for_chromosome(
                            &mut chromosome,
                            &thread_genotype,
                            fitness_cache,
                        );
                        // ignore send errors: the consumer drains until producers drop
                        let _ = sender.send((chromosome, now.elapsed()));
                    });
            });

            // a plain for loop (not `.for_each(closure)`) so the per-chromosome stop check can call
            // the whole-&self is_finished_* methods; a closure would force a whole-self capture that
            // conflicts with the `fitness_cache` borrow held by the producer above
            for (chromosome, fitness_duration) in receiver.iter() {
                self.state.increment_generation();
                self.state.chromosome.replace(chromosome);
                self.state.update_best_chromosome_and_report(
                    &self.genotype,
                    &self.config,
                    &mut self.reporter,
                );
                self.state
                    .add_duration(StrategyAction::Fitness, fitness_duration);
                self.reporter
                    .on_generation_complete(&self.genotype, &self.state, &self.config);
                if self.is_conclusive() {
                    stop.store(true, Ordering::Relaxed);
                }
            }
        });
    }
}

impl StrategyConfig for PermutateConfig {
    fn fitness_ordering(&self) -> FitnessOrdering {
        self.fitness_ordering
    }
    fn par_fitness(&self) -> bool {
        self.par_fitness
    }
    fn replace_on_equal_fitness(&self) -> bool {
        self.replace_on_equal_fitness
    }
    fn variant(&self) -> StrategyVariant {
        StrategyVariant::Permutate(self.variant)
    }
}

impl<G: PermutateGenotype> StrategyState<G> for PermutateState<G> {
    fn chromosome_as_ref(&self) -> &Option<Chromosome<G::Allele>> {
        &self.chromosome
    }
    fn population_as_ref(&self) -> &Population<G::Allele> {
        &self.population
    }
    fn chromosome_as_mut(&mut self) -> &mut Option<Chromosome<G::Allele>> {
        &mut self.chromosome
    }
    fn population_as_mut(&mut self) -> &mut Population<G::Allele> {
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
    fn best_genes(&self) -> Option<Genes<G::Allele>> {
        self.best_chromosome.as_ref().map(|c| c.genes().clone())
    }
}

impl<G: PermutateGenotype> PermutateState<G> {
    fn update_best_chromosome_and_report<SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        config: &PermutateConfig,
        reporter: &mut SR,
    ) {
        if let Some(chromosome) = self.chromosome.as_ref() {
            let now = Instant::now();
            match self.is_better_chromosome(
                chromosome,
                &config.fitness_ordering,
                config.replace_on_equal_fitness,
            ) {
                (true, true) => {
                    self.best_generation = self.current_generation;
                    self.best_fitness_score = chromosome.fitness_score();
                    self.best_chromosome = Some(chromosome.clone());
                    reporter.on_new_best_chromosome(genotype, self, config);
                    self.reset_stale_generations();
                }
                (true, false) => {
                    self.best_chromosome = Some(chromosome.clone());
                    reporter.on_new_best_chromosome_equal_fitness(genotype, self, config);
                    self.increment_stale_generations()
                }
                _ => self.increment_stale_generations(),
            }
            self.add_duration(StrategyAction::UpdateBestChromosome, now.elapsed());
        }
    }
    fn scale(&mut self, genotype: &mut G, _config: &PermutateConfig) {
        if genotype.increment_scale_index() {
            self.reset_scale_generation();
            self.reset_stale_generations();
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
            .map(|o| !o.allows_permutation())
            .unwrap()
        {
            Err(TryFromPermutateBuilderError(
                "The Genotype's mutation_type does not allow permutation. RangeGenotype/MultiRangeGenotype require MutationType::Step, StepScaled, or Discrete for permutation",
            ))
        } else {
            let genotype = builder.genotype.unwrap();
            let state = PermutateState::new(&genotype);

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),

                config: PermutateConfig {
                    fitness_ordering: builder.fitness_ordering,
                    par_fitness: builder.par_fitness,
                    replace_on_equal_fitness: builder.replace_on_equal_fitness,
                    target_fitness_score: builder.target_fitness_score,
                    abort_flag: builder.abort_flag,
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
            par_fitness: false,
            replace_on_equal_fitness: true,
            target_fitness_score: None,
            abort_flag: None,
        }
    }
}
impl PermutateConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: PermutateGenotype> PermutateState<G> {
    pub fn new(genotype: &G) -> Self {
        Self {
            current_iteration: 0,
            current_generation: 0,
            stale_generations: 0,
            scale_generation: 0,
            best_generation: 0,
            best_fitness_score: None,
            chromosome: None,
            population: Population::new_empty(genotype.chromosome_recycling()),
            durations: HashMap::new(),
            best_chromosome: None,
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
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  par_fitness: {:?}", self.par_fitness)
    }
}

impl<G: PermutateGenotype> fmt::Display for PermutateState<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_state:")?;
        writeln!(f, "  current iteration: -")?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())
    }
}
