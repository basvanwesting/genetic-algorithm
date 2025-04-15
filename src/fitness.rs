//! The search goal to optimize towards (maximize or minimize).
//!
//! Each problem will usually have its own specific [Fitness] function, therefore you need to
//! implement it yourself. Because the [Fitness] function is specific, it is also bound to the
//! [Genotype] through a trait attribute (no reason to make it generic, as the client implements for
//! a single [Genotype] type).
//!
//! See [Fitness] Trait for examples and further documentation
pub mod cache;
pub mod placeholders;
pub mod prelude;

pub use self::cache::CacheReference as FitnessCacheReference;

use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::{StrategyAction, StrategyConfig, StrategyState};
use rayon::prelude::*;
use std::cell::RefCell;
use std::time::Instant;
use thread_local::ThreadLocal;

/// Use isize for easy handling of scores (ordering, comparing) as floats are tricky in that regard.
pub type FitnessValue = isize;

#[derive(Copy, Clone, Debug)]
pub enum FitnessOrdering {
    Maximize,
    Minimize,
}

/// This is just a shortcut for `Self::Genotype`
pub type FitnessGenotype<F> = <F as Fitness>::Genotype;
/// This is just a shortcut for `<Self::Genotype as Genotype>::Chromosome`
pub type FitnessChromosome<F> = <<F as Fitness>::Genotype as Genotype>::Chromosome;
/// This is just a shortcut for `Population<<Self::Genotype as Genotype::Chromosome>`
pub type FitnessPopulation<F> = Population<<<F as Fitness>::Genotype as Genotype>::Chromosome>;

/// The fitness function, is implemented as a fitness method object.
///
/// Normally the fitness returns [`Some(FitnessValue)`](FitnessValue) for the chromosome, which can be minimized or
/// maximized in the search strategy (e.g. [Evolve](crate::strategy::evolve::Evolve)) by providing the [FitnessOrdering].
///
/// If the fitness returns `None`, the chromosome is assumed invalid and taken last in the [selection](crate::select) phase (also when minimizing).
///
/// # User implementation
///
/// There are two possible levels to implement. At least one level needs to be implemented:
/// * [`calculate_for_chromosome(...) -> Option<FitnessValue>`](Fitness::calculate_for_chromosome)
///   * The standard situation, suits all strategies. Implementable with all Genotypes.
///   * Standard [Genotype]s have [GenesOwner](crate::chromosome::GenesOwner) chromosomes. These
///     chromosomes have a `genes` field, which can be read for the calculations.
///   * non-standard [Genotype]s with [GenesPointer](crate::chromosome::GenesPointer) chromosomes.
///     These chromosomes have don't have a `genes` field, so you need to retrieve the genes using
///     [genotype.genes_slice(&chromosome)](crate::genotype::Genotype::genes_slice), which can then
///     be read for the calculations. But for these types you usually don't want to reach this call
///     level, see other level below
/// * [`calculate_for_population(...) -> Vec<Option<FitnessValue>>`](Fitness::calculate_for_population)
///   * *Only overwrite for matrix Genotypes (designed for possible GPU acceleration)*
///   * If not overwritten, results in calling
///     [calculate_for_chromosome](Fitness::calculate_for_chromosome) for each chromosome in the
///     population. So it doesn't have to be implemented by default, but it is a possible point to
///     intercept with a custom implementation where the whole population data is available.
///   * Only for [Genotype] with [GenesPointer](crate::chromosome::GenesPointer) chromosomes. These
///     chromosomes don't have a `genes` field to read, but a `row_id`. The matrix [Genotype] has a contiguous
///     memory `data` field with all the data, which can be calculated in one go.
///     * [DynamicMatrixGenotype](crate::genotype::DynamicMatrixGenotype)
///     * [StaticMatrixGenotype](crate::genotype::StaticMatrixGenotype)
///   * The order and length of the rows in the genotype data matrix needs to be preserved in the
///     returned vector as it matches the row_id on the chromosome
///   * The order and length of the population does not matter at all and will most likely not align.
///     The population is provided, because the full genotype matrix data also contains untainted
///     chromosomes which already have a fitness_score (and will not be updated). The results for
///     these chromosomes will be ignored, thus these do not have to be recalculated, so knowing
///     which ones might be relevant (or not). The order of the results still need to align, so if the
///     calculation is skipped, a `None` value should be inserted in the results to keep the order
///     and length aligned.
///
/// The strategies use different levels of calls in [Fitness]. So you cannot always just intercept at
/// [calculate_for_population](Fitness::calculate_for_population) and be sure
/// [calculate_for_chromosome](Fitness::calculate_for_chromosome) will not be called:
///
/// * Population level calculations (calling
///   [calculate_for_chromosome](Fitness::calculate_for_chromosome) indirectly through
///   [calculate_for_population](Fitness::calculate_for_population), if not overwritten)
///   * [Evolve](crate::strategy::evolve::Evolve)
///   * [HillClimb](crate::strategy::hill_climb::HillClimb) with [SteepestAscent](crate::strategy::hill_climb::HillClimbVariant::SteepestAscent)
/// * Chromosome level calculations (calling
///   [calculate_for_chromosome](Fitness::calculate_for_chromosome) directly, bypassing
///   [calculate_for_population](Fitness::calculate_for_population) entirely)
///   * [Permutate](crate::strategy::permutate::Permutate)
///   * [HillClimb](crate::strategy::hill_climb::HillClimb) with [Stochastic](crate::strategy::hill_climb::HillClimbVariant::Stochastic)
///
/// Therefore, additionally, you might need to implement
/// [calculate_for_chromosome](Fitness::calculate_for_chromosome) for
/// [GenesPointer](crate::chromosome::GenesPointer) chromosomes. This is sometimes needed when
/// testing out different strategies with different call levels. Problably no longer needed once
/// settled on a strategy.
///
/// # Panics
///
/// [calculate_for_chromosome](Fitness::calculate_for_chromosome) has a default implementation which panics, because it doesn't need to
/// be implemented for genotypes which implement [calculate_for_population](Fitness::calculate_for_population). Will panic if reached and not implemented.
///
/// # Example (calculate_for_chromosome, standard GenesOwner chromosome):
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct CountTrue;
/// impl Fitness for CountTrue {
///     type Genotype = BinaryGenotype;
///     fn calculate_for_chromosome(
///         &mut self,
///         chromosome: &FitnessChromosome<Self>,
///         _genotype: &FitnessGenotype<Self>
///     ) -> Option<FitnessValue> {
///         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
///     }
/// }
/// ```
///
/// # Example (calculate_for_population, static matrix calculation, GenesPointer chromosome):
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
/// use genetic_algorithm::strategy::evolve::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct SumStaticMatrixGenes;
/// impl Fitness for SumStaticMatrixGenes {
///     type Genotype = StaticMatrixGenotype<u16, 10, 100>;
///     fn calculate_for_population(
///         &mut self,
///         population: &Population<StaticMatrixChromosome>,
///         genotype: &FitnessGenotype<Self>,
///     ) -> Vec<Option<FitnessValue>> {
///         // pure matrix data calculation on [[T; N] M]
///         // the order of the rows needs to be preserved as it matches the row_id on the chromosome
///         // the order of the population does not matter at all and will most likely not align
///         genotype
///             .data
///             .iter()
///             .map(|genes| {
///                 genes
///                     .iter()
///                     .sum::<u16>() as FitnessValue
///             })
///             .map(Some)
///             .collect()
///     }
/// }
/// ```
///
/// # Example (calculate_for_population, dynamic matrix calculation, GenesPointer chromosome):
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
/// use genetic_algorithm::strategy::evolve::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct SumDynamicMatrixGenes;
/// impl Fitness for SumDynamicMatrixGenes {
///     type Genotype = DynamicMatrixGenotype<u16>;
///     fn calculate_for_population(
///         &mut self,
///         population: &Population<DynamicMatrixChromosome>,
///         genotype: &FitnessGenotype<Self>,
///     ) -> Vec<Option<FitnessValue>> {
///         // pure matrix data calculation on Vec<T> with genes_size step
///         // the order of the rows needs to be preserved as it matches the row_id on the chromosome
///         // the order of the population does not matter at all and will most likely not align
///         genotype
///             .data
///             .chunks(genotype.genes_size())
///             .map(|genes| {
///                 genes
///                     .iter()
///                     .sum::<u16>() as FitnessValue
///             })
///             .map(Some)
///             .collect()
///     }
/// }
/// ```
///
/// # Example (calculate_for_chromosome, matrix fall back, GenesPointer chromosome):
/// *Note: For exploration purposes when switching stratgies a lot, not really used in final implementation*
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
/// use genetic_algorithm::strategy::hill_climb::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct SumStaticMatrixGenes;
/// impl Fitness for SumStaticMatrixGenes {
///     type Genotype = StaticMatrixGenotype<u16, 10, 100>;
///     fn calculate_for_chromosome(
///         &mut self,
///         chromosome: &FitnessChromosome<Self>,
///         genotype: &Self::Genotype,
///     ) -> Option<FitnessValue> {
///         let score = genotype.genes_slice(chromosome).iter().sum::<u16>();
///         Some(score as FitnessValue)
///     }
/// }
///
pub trait Fitness: Clone + Send + Sync + std::fmt::Debug {
    type Genotype: Genotype;
    fn call_for_state_population<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &mut S,
        config: &C,
        thread_local: Option<&ThreadLocal<RefCell<Self>>>,
    ) {
        let now = Instant::now();
        self.call_for_population(
            state.population_as_mut(),
            genotype,
            thread_local,
            config.fitness_cache_reference(),
        );
        state.add_duration(StrategyAction::Fitness, now.elapsed());
    }
    fn call_for_state_chromosome<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &mut S,
        config: &C,
    ) {
        if let Some(chromosome) = state.chromosome_as_mut() {
            let now = Instant::now();
            self.call_for_chromosome(chromosome, genotype, config.fitness_cache_reference());
            state.add_duration(StrategyAction::Fitness, now.elapsed());
        }
    }
    /// Pass thread_local for external control of fitness state in multithreading
    fn call_for_population(
        &mut self,
        population: &mut FitnessPopulation<Self>,
        genotype: &Self::Genotype,
        thread_local: Option<&ThreadLocal<RefCell<Self>>>,
        cache_reference: Option<&FitnessCacheReference>,
    ) {
        let fitness_scores = self.calculate_for_population(population, genotype);
        if fitness_scores.is_empty() {
            if let Some(thread_local) = thread_local {
                population
                    .chromosomes
                    .par_iter_mut()
                    .filter(|c| c.fitness_score().is_none())
                    .for_each_init(
                        || {
                            thread_local
                                .get_or(|| std::cell::RefCell::new(self.clone()))
                                .borrow_mut()
                        },
                        |fitness, chromosome| {
                            fitness.call_for_chromosome(chromosome, genotype, cache_reference);
                        },
                    );
            } else {
                population
                    .chromosomes
                    .iter_mut()
                    .filter(|c| c.fitness_score().is_none())
                    .for_each(|c| self.call_for_chromosome(c, genotype, cache_reference));
            }
        } else {
            genotype.update_population_fitness_scores(population, fitness_scores);
        }
    }
    fn call_for_chromosome(
        &mut self,
        chromosome: &mut FitnessChromosome<Self>,
        genotype: &Self::Genotype,
        cache_reference: Option<&FitnessCacheReference>,
    ) {
        let value = match (cache_reference, chromosome.genes_hash()) {
            (Some(cache), Some(genes_hash)) => {
                if let Some(value) = cache.read(genes_hash) {
                    Some(value)
                } else if let Some(value) = self.calculate_for_chromosome(chromosome, genotype) {
                    cache.write(genes_hash, value);
                    Some(value)
                } else {
                    None
                }
            }
            _ => self.calculate_for_chromosome(chromosome, genotype),
        };
        chromosome.set_fitness_score(value);
    }
    /// Optional interception point for client implementation.
    ///
    /// The order and length of the results need to align with the order and length of the genotype data matrix.
    /// The order and length of the population does not matter at all and will most likely not align.
    fn calculate_for_population(
        &mut self,
        _population: &FitnessPopulation<Self>,
        _genotype: &Self::Genotype,
    ) -> Vec<Option<FitnessValue>> {
        Vec::new()
    }
    /// Optional interception point for client implementation
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        panic!("Implement calculate_for_chromosome for your Fitness (or higher in the call stack when using StaticMatrixGenotype)");
    }
}
