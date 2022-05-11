use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::BinaryGenotype;

/// placeholder for internal testing, not really used in practice
#[derive(Clone, Debug)]
pub struct CountTrue;
impl Fitness for CountTrue {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}
