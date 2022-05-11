use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{
    ContinuousGenotype, DiscreteGenotype, Genotype, MultiDiscreteGenotype, UniqueDiscreteGenotype,
};

/// placeholder for internal testing, not really used in practice
#[derive(Clone, Debug)]
pub struct SumContinuousGenotype;
impl Fitness for SumContinuousGenotype {
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

/// placeholder for internal testing, not really used in practice
#[derive(Clone, Debug)]
pub struct SumDiscreteGenotype;
impl Fitness for SumDiscreteGenotype {
    type Genotype = DiscreteGenotype<usize>;
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

/// placeholder for internal testing, not really used in practice
#[derive(Clone, Debug)]
pub struct SumUniqueDiscreteGenotype;
impl Fitness for SumUniqueDiscreteGenotype {
    type Genotype = UniqueDiscreteGenotype<usize>;
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

/// placeholder for internal testing, not really used in practice
#[derive(Clone, Debug)]
pub struct SumMultiDiscreteGenotype;
impl Fitness for SumMultiDiscreteGenotype {
    type Genotype = MultiDiscreteGenotype<usize>;
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
