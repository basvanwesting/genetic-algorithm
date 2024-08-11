//! placeholders for testing and bootstrapping, not really used in practice
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{Allele, BinaryAllele, ContinuousAllele};
use std::marker::PhantomData;

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct Zero<T: Allele>(PhantomData<T>);
impl<T: Allele> Zero<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T: Allele> Default for Zero<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Allele> Fitness for Zero<T> {
    type Allele = T;
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &Chromosome<Self::Allele>,
    ) -> Option<FitnessValue> {
        Some(0)
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct CountTrue;
impl Fitness for CountTrue {
    type Allele = BinaryAllele;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumContinuousAllele(pub f32);
impl Fitness for SumContinuousAllele {
    type Allele = ContinuousAllele;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .map(|v| v / self.0)
                .sum::<Self::Allele>() as FitnessValue,
        )
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumDiscreteAllele;
impl Fitness for SumDiscreteAllele {
    type Allele = usize;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().sum::<Self::Allele>() as FitnessValue)
    }
}
