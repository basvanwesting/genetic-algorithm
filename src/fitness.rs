use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;

pub type FitnessValue = isize;

#[derive(Copy, Clone)]
pub enum FitnessOrdering {
    Maximize,
    Minimize,
}

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

mod simple_count;
pub use self::simple_count::SimpleCount as FitnessSimpleCount;

mod simple_sum;
pub use self::simple_sum::SimpleSumContinuousGenotype as FitnessSimpleSumContinuousGenotype;
pub use self::simple_sum::SimpleSumIndexGenotype as FitnessSimpleSumIndexGenotype;
pub use self::simple_sum::SimpleSumMultiIndexGenotype as FitnessSimpleSumMultiIndexGenotype;
pub use self::simple_sum::SimpleSumUniqueIndexGenotype as FitnessSimpleSumUniqueIndexGenotype;
