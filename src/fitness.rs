use crate::chromosome::Chromosome;
use crate::genotype::{
    BinaryGenotype, ContinuousGenotype, Genotype, IndexGenotype, UniqueIndexGenotype,
};
use crate::population::Population;

pub trait Fitness: Clone + std::fmt::Debug {
    type Genotype: Genotype;
    fn call_for_population(
        &self,
        mut population: Population<Self::Genotype>,
    ) -> Population<Self::Genotype> {
        population
            .chromosomes
            .iter_mut()
            .filter(|c| c.fitness_score.is_none())
            .for_each(|c| c.fitness_score = Some(self.call_for_chromosome(c)));
        population
    }
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize;
}

#[derive(Clone, Debug)]
pub struct SimpleSumBinaryGenotype;
impl Fitness for SimpleSumBinaryGenotype {
    type Genotype = BinaryGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        chromosome.genes.iter().filter(|&value| *value).count() as isize
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumContinuousGenotype;
impl Fitness for SimpleSumContinuousGenotype {
    type Genotype = ContinuousGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        chromosome
            .genes
            .iter()
            .sum::<<Self::Genotype as Genotype>::Gene>() as isize
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumIndexGenotype;
impl Fitness for SimpleSumIndexGenotype {
    type Genotype = IndexGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        chromosome
            .genes
            .iter()
            .sum::<<Self::Genotype as Genotype>::Gene>() as isize
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumUniqueIndexGenotype;
impl Fitness for SimpleSumUniqueIndexGenotype {
    type Genotype = UniqueIndexGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        chromosome
            .genes
            .iter()
            .sum::<<Self::Genotype as Genotype>::Gene>() as isize
    }
}
