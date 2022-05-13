//! placeholders for testing and bootstrapping, not really used in practice
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{
    BinaryGenotype, ContinuousGenotype, DiscreteGenotype, Genotype, MultiContinuousGenotype,
    MultiDiscreteGenotype, UniqueDiscreteGenotype,
};

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct CountTrue;
impl Fitness for CountTrue {
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumContinuousGenotype;
impl Fitness for SumContinuousGenotype {
    type Genotype = ContinuousGenotype;
    fn calculate_for_chromosome(
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

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumDiscreteGenotype;
impl Fitness for SumDiscreteGenotype {
    type Genotype = DiscreteGenotype;
    fn calculate_for_chromosome(
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

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumMultiContinuousGenotype;
impl Fitness for SumMultiContinuousGenotype {
    type Genotype = MultiContinuousGenotype;
    fn calculate_for_chromosome(
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

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumMultiDiscreteGenotype;
impl Fitness for SumMultiDiscreteGenotype {
    type Genotype = MultiDiscreteGenotype;
    fn calculate_for_chromosome(
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

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumUniqueDiscreteGenotype;
impl Fitness for SumUniqueDiscreteGenotype {
    type Genotype = UniqueDiscreteGenotype;
    fn calculate_for_chromosome(
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
