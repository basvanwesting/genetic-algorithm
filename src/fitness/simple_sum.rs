use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{
    ContinuousGenotype, Genotype, IndexGenotype, MultiIndexGenotype, UniqueIndexGenotype,
};

#[derive(Clone, Debug)]
pub struct SimpleSumContinuousGenotype;
impl Fitness for SimpleSumContinuousGenotype {
    type Genotype = ContinuousGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Gene>() as FitnessValue,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumIndexGenotype;
impl Fitness for SimpleSumIndexGenotype {
    type Genotype = IndexGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Gene>() as FitnessValue,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumUniqueIndexGenotype;
impl Fitness for SimpleSumUniqueIndexGenotype {
    type Genotype = UniqueIndexGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Gene>() as FitnessValue,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumMultiIndexGenotype;
impl Fitness for SimpleSumMultiIndexGenotype {
    type Genotype = MultiIndexGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Gene>() as FitnessValue,
        )
    }
}
