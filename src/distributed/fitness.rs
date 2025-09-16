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

pub use self::cache::Cache as FitnessCache;

use crate::distributed::chromosome::Chromosome;
use crate::distributed::genotype::Genotype;
use crate::distributed::population::Population;
use crate::distributed::strategy::{StrategyAction, StrategyConfig, StrategyState};
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
/// This is just a shortcut for `Chromosome<<Self::Genotype as Genotype>::Allele>`
pub type FitnessChromosome<F> = Chromosome<<<F as Fitness>::Genotype as Genotype>::Allele>;
/// This is just a shortcut for `Vec<<Self::Genotype as Genotype>::Allele>`
pub type FitnessGenes<F> = Vec<<<F as Fitness>::Genotype as Genotype>::Allele>;
/// This is just a shortcut for `Population<<Self::Genotype as Genotype>::Allele>`
pub type FitnessPopulation<F> = Population<<<F as Fitness>::Genotype as Genotype>::Allele>;

/// The fitness function, is implemented as a fitness method object.
///
/// Normally the fitness returns [`Some(FitnessValue)`](FitnessValue) for the chromosome, which can be minimized or
/// maximized in the search strategy (e.g. [Evolve](crate::strategy::evolve::Evolve)) by providing the [FitnessOrdering].
///
/// If the fitness returns `None`, the chromosome is assumed invalid and taken last in the [selection](crate::select) phase (also when minimizing).
///
/// # User implementation
///
/// You must implement [`calculate_for_chromosome(...) -> Option<FitnessValue>`](Fitness::calculate_for_chromosome)
/// which calculates the fitness for a single chromosome.
/// Distributed [Genotype]s have [GenesOwner](crate::chromosome::GenesOwner) chromosomes. These
/// chromosomes have a `genes` field, which can be read for the calculations.
///
/// Fitness uses &mut self for performance because it dominates the runtime. Preparing memory
/// allocations on initialization and reusing them for each chromosome can really impact
/// performance. For parallel evaluation, each thread gets its own clone via ThreadLocal.
///
/// # Example (calculate_for_chromosome, standard GenesOwner chromosome):
/// ```rust
/// use genetic_algorithm::distributed::fitness::prelude::*;
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
            config.fitness_cache(),
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
            self.call_for_chromosome(chromosome, genotype, config.fitness_cache());
            state.add_duration(StrategyAction::Fitness, now.elapsed());
        }
    }
    /// Pass thread_local for external control of fitness state in multithreading
    fn call_for_population(
        &mut self,
        population: &mut FitnessPopulation<Self>,
        genotype: &Self::Genotype,
        thread_local: Option<&ThreadLocal<RefCell<Self>>>,
        cache: Option<&FitnessCache>,
    ) {
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
                        fitness.call_for_chromosome(chromosome, genotype, cache);
                    },
                );
        } else {
            population
                .chromosomes
                .iter_mut()
                .filter(|c| c.fitness_score().is_none())
                .for_each(|c| self.call_for_chromosome(c, genotype, cache));
        }
    }
    fn call_for_chromosome(
        &mut self,
        chromosome: &mut FitnessChromosome<Self>,
        genotype: &Self::Genotype,
        cache: Option<&FitnessCache>,
    ) {
        let value = match (cache, chromosome.genes_hash()) {
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
    /// Must be implemented by client
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        genotype: &Self::Genotype,
    ) -> Option<FitnessValue>;
}
