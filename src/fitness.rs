//! The search goal to optimize towards (maximize or minimize).
//!
//! Each problem will usually have its own specific [Fitness] function, therefore you need to
//! implement it yourself. Because the [Fitness] function is specific, it is also bound to the
//! [Allele] through a trait attribute (no reason to make it generic, as the client implements for
//! a single [Allele] type).
//!
//! See [Fitness] Trait
pub mod placeholders;
pub mod prelude;

use crate::chromosome::Chromosome;
use crate::genotype::Allele;
use crate::population::Population;
use rayon::prelude::*;
use std::cell::RefCell;
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
/// If the fitness returns `None`, the chromosome is assumed invalid and taken last in the [competition](crate::compete) phase (also when minimizing).
///
/// # Example:
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct CountTrue;
/// impl Fitness for CountTrue {
///     type Allele = BinaryAllele;
///     fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<Self::Allele>) -> Option<FitnessValue> {
///         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
///     }
/// }
/// ```
pub trait Fitness: Clone + Send + Sync + std::fmt::Debug {
    type Allele: Allele;
    /// pass thread_local for external control of fitness caching in multithreading
    fn call_for_population(
        &mut self,
        population: &mut Population<Self::Allele>,
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
                        fitness.call_for_chromosome(chromosome);
                    },
                );
        } else {
            population
                .chromosomes
                .iter_mut()
                .filter(|c| c.fitness_score.is_none())
                .for_each(|c| self.call_for_chromosome(c));
        }
    }
    fn call_for_chromosome(&mut self, chromosome: &mut Chromosome<Self::Allele>) {
        chromosome.fitness_score = self.calculate_for_chromosome(chromosome);
    }
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
    ) -> Option<FitnessValue>;
}
