use crate::chromosome::Chromosome;
use crate::fitness::Fitness;
use crate::genotype::BinaryGenotype;

#[derive(Clone, Debug)]
pub struct SimpleCount;
impl Fitness for SimpleCount {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        chromosome.genes.iter().filter(|&value| *value).count() as isize
    }
}
