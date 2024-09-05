//! The search goal to optimize towards (maximize or minimize).
//!
//! Each problem will usually have its own specific [Fitness] function, therefore you need to
//! implement it yourself. Because the [Fitness] function is specific, it is also bound to the
//! [Genotype] through a trait attribute (no reason to make it generic, as the client implements for
//! a single [Genotype] type).
//!
//! See [Fitness] Trait
pub mod placeholders;
pub mod prelude;

use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::{StrategyAction, StrategyState};
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

/// The fitness function, is implemented as a fitness method object.
///
/// Normally the fitness returns [`Some(FitnessValue)`](FitnessValue) for the chromosome, which can be minimized or
/// maximized in the search strategy (e.g. [Evolve](crate::strategy::evolve::Evolve) or
/// [Permutate](crate::strategy::permutate::Permutate)) by providing the [FitnessOrdering].
///
/// If the fitness returns `None`, the chromosome is assumed invalid and taken last in the [selection](crate::select) phase (also when minimizing).
///
/// # Example:
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct CountTrue;
/// impl Fitness for CountTrue {
///     type Genotype = BinaryGenotype;
///     fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>, _genotype: &Self::Genotype) -> Option<FitnessValue> {
///         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
///     }
/// }
/// ```
pub trait Fitness: Clone + Send + Sync + std::fmt::Debug {
    type Genotype: Genotype;
    fn call_for_state_population<S: StrategyState<Self::Genotype>>(
        &mut self,
        state: &mut S,
        genotype: &mut Self::Genotype,
        thread_local: Option<&ThreadLocal<RefCell<Self>>>,
    ) {
        let now = Instant::now();
        self.call_for_population(state.population_as_mut(), genotype, thread_local);
        state.add_duration(StrategyAction::Fitness, now.elapsed());
    }
    fn call_for_state_chromosome<S: StrategyState<Self::Genotype>>(
        &mut self,
        state: &mut S,
        genotype: &Self::Genotype,
    ) {
        let now = Instant::now();
        self.call_for_chromosome(state.chromosome_as_mut(), genotype);
        state.add_duration(StrategyAction::Fitness, now.elapsed());
    }
    /// Implement by Client for MatrixGenotype
    /// pass thread_local for external control of fitness caching in multithreading
    fn call_for_population(
        &mut self,
        population: &mut Population<Self::Genotype>,
        genotype: &mut Self::Genotype,
        thread_local: Option<&ThreadLocal<RefCell<Self>>>,
    ) {
        if let Some(thread_local) = thread_local {
            population
                .chromosomes
                .par_iter_mut()
                .filter(|c| c.fitness_score.is_none())
                .for_each_init(
                    || {
                        thread_local
                            .get_or(|| std::cell::RefCell::new(self.clone()))
                            .borrow_mut()
                    },
                    |fitness, chromosome| {
                        fitness.call_for_chromosome(chromosome, genotype);
                    },
                );
        } else {
            population
                .chromosomes
                .iter_mut()
                .filter(|c| c.fitness_score.is_none())
                .for_each(|c| self.call_for_chromosome(c, genotype));
        }
    }
    fn call_for_chromosome(
        &mut self,
        chromosome: &mut Chromosome<Self::Genotype>,
        genotype: &Self::Genotype,
    ) {
        if !genotype.chromosome_is_empty(chromosome) {
            chromosome.fitness_score = self.calculate_for_chromosome(chromosome, genotype);
        }
    }
    /// Implement by Client for normal Genotypes
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &Chromosome<Self::Genotype>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        panic!("Implement calculate_for_chromosome for your Fitness (or higher in the call stack when using MatrixGenotype)");
    }
}
