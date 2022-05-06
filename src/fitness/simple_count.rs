use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::BinaryGenotype;

#[derive(Clone, Debug)]
pub struct SimpleCount;
impl Fitness for SimpleCount {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}
