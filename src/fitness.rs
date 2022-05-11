//! The search goal to optimize towards (maximize or minimize).
//!
//! Each problem will usually have its own specific [Fitness] function, therefore you need to
//! implement it yourself. Because the [Fitness] function is specific, it is also bound to the
//! [genotype](crate::genotype) through a trait attribute (no reason to make it generic).
pub mod prelude;
mod simple_count;
mod simple_sum;

pub use self::simple_count::SimpleCount as FitnessSimpleCount;
pub use self::simple_sum::SimpleSumContinuousGenotype as FitnessSimpleSumContinuousGenotype;
pub use self::simple_sum::SimpleSumDiscreteGenotype as FitnessSimpleSumDiscreteGenotype;
pub use self::simple_sum::SimpleSumMultiDiscreteGenotype as FitnessSimpleSumMultiDiscreteGenotype;
pub use self::simple_sum::SimpleSumUniqueDiscreteGenotype as FitnessSimpleSumUniqueDiscreteGenotype;

use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;

pub type FitnessValue = isize;

#[derive(Copy, Clone, Debug)]
pub enum FitnessOrdering {
    Maximize,
    Minimize,
}

/// The fitness function, is implemented as a fitness method object.
///
/// Normally the fitness returns `Some(FitnessValue)` for the chromosome, which can be minimized or
/// maximized in the search strategy (e.g. [Evolve](crate::evolve::Evolve) or
/// [Permutate](crate::permutate::Permutate)) by providing the [FitnessOrdering].
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
///     type Genotype = BinaryGenotype;
///     fn call_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
///         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
///     }
/// }
/// ```
pub trait Fitness: Clone + std::fmt::Debug {
    type Genotype: Genotype;
    fn call_for_population(
        &mut self,
        mut population: Population<Self::Genotype>,
    ) -> Population<Self::Genotype> {
        population
            .chromosomes
            .iter_mut()
            .filter(|c| c.fitness_score.is_none())
            .for_each(|c| c.fitness_score = self.call_for_chromosome(c));
        population
    }
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue>;
}
