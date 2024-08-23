//! A solution strategy for finding the best chromosome in case of small problem spaces (with a 100% guarantee)
mod builder;
pub mod prelude;
mod reporter;

pub use self::builder::{
    Builder as PermutateBuilder, TryFromBuilderError as TryFromPermutateBuilderError,
};

use super::{Strategy, StrategyConfig, StrategyState};
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::{Allele, PermutableGenotype};
use num::BigUint;
use rand::Rng;
use rayon::prelude::*;
use std::fmt;
use std::sync::mpsc::sync_channel;

pub use self::reporter::Log as PermutateReporterLog;
pub use self::reporter::Noop as PermutateReporterNoop;
pub use self::reporter::Reporter as PermutateReporter;
pub use self::reporter::Simple as PermutateReporterSimple;

/// All possible combinations of genes are iterated over as chromosomes.
/// The fitness is calculated for each chromosome and the best is taken.
/// For efficiency reasons the full population is never instantiated as a whole.
///
/// The `chromosome_permutations_size` is subject to combinatorial explosion, so check the genotype
/// for practical values before using the [Permutate] strategy. This will not pose any memory
/// issues, as the permutations are not instantiated at the same time, just iterated over. But it
/// will take forever...
///
/// There are reporting hooks in the loop receiving the [PermutateState], which can by handled by an
/// [PermutateReporter] (e.g. [PermutateReporterNoop], [PermutateReporterSimple]). But you are encouraged to
/// roll your own, see [PermutateReporter].
///
/// See [PermutateBuilder] for initialization options.
///
/// All multithreading mechanisms are implemented using [rayon::iter] and [std::sync::mpsc].
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::permutate::prelude::*;
/// use genetic_algorithm::fitness::placeholders::CountTrue;
/// use rand::prelude::*;
/// use rand::rngs::SmallRng;
///
/// // the search space
/// let genotype = BinaryGenotype::builder() // boolean alleles
///     .with_genes_size(16)                 // 16 genes per chromosome
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let mut rng = SmallRng::from_entropy(); // a randomness provider implementing Trait rand::Rng + Clone + Send + Sync
/// let permutate = Permutate::builder()
///     .with_genotype(genotype)
///     .with_fitness(CountTrue)                          // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_par_fitness(true)                           // optional, defaults to false, use parallel fitness calculation
///     .with_reporter(PermutateReporterSimple::new(100)) // optional builder step, report every 100 generations
///     .call(&mut rng)
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = permutate.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes, vec![false; 16])
/// ```
pub struct Permutate<
    G: PermutableGenotype,
    F: Fitness<Allele = G::Allele>,
    SR: PermutateReporter<Allele = G::Allele>,
> {
    genotype: G,
    fitness: F,
    pub config: PermutateConfig,
    pub state: PermutateState<G::Allele>,
    reporter: SR,
}

pub struct PermutateConfig {
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
}

/// Stores the state of the Permutate strategy. Next to the expected general fields, the following
/// strategy specific fields are added:
/// * total_population_size: only the size as the full population is never instantiated simultaneously
pub struct PermutateState<A: Allele> {
    pub current_iteration: usize,
    pub current_generation: usize,
    pub stale_generations: usize,
    pub best_generation: usize,
    pub best_chromosome: Option<Chromosome<A>>,

    pub total_population_size: BigUint,
}

impl<
        G: PermutableGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: PermutateReporter<Allele = G::Allele>,
    > Strategy<G> for Permutate<G, F, SR>
{
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.reporter
            .on_start(&self.genotype, &self.state, &self.config);
        if self.config.par_fitness {
            self.call_parallel(rng)
        } else {
            self.call_sequential(rng)
        }
        self.reporter.on_finish(&self.state, &self.config);
    }
    fn best_chromosome(&self) -> Option<Chromosome<G::Allele>> {
        self.state.best_chromosome()
    }
    fn best_generation(&self) -> usize {
        self.state.best_generation
    }
    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.state.best_fitness_score()
    }
}

impl<G: PermutableGenotype, F: Fitness<Allele = G::Allele>>
    Permutate<G, F, PermutateReporterNoop<G::Allele>>
{
    pub fn builder() -> PermutateBuilder<G, F, PermutateReporterNoop<G::Allele>> {
        PermutateBuilder::new()
    }
}

impl<
        G: PermutableGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: PermutateReporter<Allele = G::Allele>,
    > Permutate<G, F, SR>
{
    fn call_sequential<R: Rng>(&mut self, _rng: &mut R) {
        self.genotype
            .clone()
            .chromosome_permutations_into_iter()
            .for_each(|mut chromosome| {
                self.state.current_generation += 1;
                self.fitness.call_for_chromosome(&mut chromosome);
                self.state.update_best_chromosome_and_report(
                    &chromosome,
                    &self.config,
                    &mut self.reporter,
                );
                self.reporter.on_new_generation(&self.state, &self.config);
            });
    }
    fn call_parallel<R: Rng>(&mut self, _rng: &mut R) {
        rayon::scope(|s| {
            let genotype = &self.genotype;
            let fitness = self.fitness.clone();
            let (sender, receiver) = sync_channel(1000);

            s.spawn(move |_| {
                genotype
                    .chromosome_permutations_into_iter()
                    .par_bridge()
                    .for_each_with((sender, fitness), |(sender, fitness), mut chromosome| {
                        fitness.call_for_chromosome(&mut chromosome);
                        sender.send(chromosome).unwrap();
                    });
            });

            receiver.iter().for_each(|chromosome| {
                self.state.current_generation += 1;
                self.state.update_best_chromosome_and_report(
                    &chromosome,
                    &self.config,
                    &mut self.reporter,
                );
            })
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
}

impl<A: Allele> StrategyState<A> for PermutateState<A> {
    fn best_chromosome(&self) -> Option<Chromosome<A>> {
        self.best_chromosome.clone()
    }
    fn best_chromosome_as_ref(&self) -> Option<&Chromosome<A>> {
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
    fn stale_generations(&self) -> usize {
        self.stale_generations
    }
    fn increment_stale_generations(&mut self) {
        self.stale_generations += 1;
    }
    fn reset_stale_generations(&mut self) {
        self.stale_generations = 0;
    }
    fn set_best_chromosome(
        &mut self,
        best_chromosome: &Chromosome<A>,
        improved_fitness: bool,
    ) -> (bool, bool) {
        self.best_chromosome = Some(best_chromosome.clone());
        if improved_fitness {
            self.best_generation = self.current_generation;
        }
        (true, improved_fitness)
    }
}

impl<A: Allele> PermutateState<A> {
    fn update_best_chromosome_and_report<SR: PermutateReporter<Allele = A>>(
        &mut self,
        contending_chromosome: &Chromosome<A>,
        config: &PermutateConfig,
        reporter: &mut SR,
    ) {
        match self.update_best_chromosome(
            contending_chromosome,
            &config.fitness_ordering,
            config.replace_on_equal_fitness,
        ) {
            (true, true) => {
                reporter.on_new_best_chromosome(self, config);
                self.reset_stale_generations();
            }
            (true, false) => {
                reporter.on_new_best_chromosome_equal_fitness(self, config);
                self.increment_stale_generations()
            }
            _ => self.increment_stale_generations(),
        }
    }
}

impl<
        G: PermutableGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: PermutateReporter<Allele = G::Allele>,
    > TryFrom<PermutateBuilder<G, F, SR>> for Permutate<G, F, SR>
{
    type Error = TryFromPermutateBuilderError;

    fn try_from(builder: PermutateBuilder<G, F, SR>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromPermutateBuilderError(
                "Permutate requires a Genotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromPermutateBuilderError("Permutate requires a Fitness"))
        } else {
            let genotype = builder.genotype.unwrap();
            let total_population_size = genotype.chromosome_permutations_size();

            Ok(Self {
                genotype,
                fitness: builder.fitness.unwrap(),

                config: PermutateConfig {
                    fitness_ordering: builder.fitness_ordering,
                    par_fitness: builder.par_fitness,
                    replace_on_equal_fitness: builder.replace_on_equal_fitness,
                },
                state: PermutateState {
                    total_population_size,
                    ..Default::default()
                },
                reporter: builder.reporter,
            })
        }
    }
}

impl Default for PermutateConfig {
    fn default() -> Self {
        Self {
            fitness_ordering: FitnessOrdering::Maximize,
            par_fitness: false,
            replace_on_equal_fitness: false,
        }
    }
}
impl PermutateConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A: Allele> Default for PermutateState<A> {
    fn default() -> Self {
        Self {
            total_population_size: BigUint::default(),
            current_iteration: 0,
            current_generation: 0,
            stale_generations: 0,
            best_generation: 0,
            best_chromosome: None,
        }
    }
}
impl<A: Allele> PermutateState<A> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<
        G: PermutableGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: PermutateReporter<Allele = G::Allele>,
    > fmt::Display for Permutate<G, F, SR>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(f, "{}", self.config)?;
        writeln!(f, "{}", self.state)
    }
}

impl fmt::Display for PermutateConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_config:")?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  par_fitness: {:?}", self.par_fitness)
    }
}

impl<A: Allele> fmt::Display for PermutateState<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "permutate_state:")?;
        writeln!(f, "  total_population_size: {}", self.total_population_size)?;
        writeln!(f, "  current iteration: -")?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
