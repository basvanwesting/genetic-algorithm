//! placeholders for testing and bootstrapping, not really used in practice
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{
    BinaryGenotype, ContinuousGenotype, DiscreteGenotype, Genotype, MultiContinuousGenotype,
    MultiContinuousGenotypeAllele, MultiDiscreteGenotype, MultiUniqueGenotype, UniqueGenotype,
};
use std::marker::PhantomData;

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct Zero<T: Genotype>(PhantomData<T>);
impl<T: Genotype> Zero<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T: Genotype> Default for Zero<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T: Genotype> Fitness for Zero<T> {
    type Genotype = T;
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(0)
    }
}

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
pub struct SumContinuousGenotype(pub f32);
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
                .map(|v| v / self.0)
                .sum::<<Self::Genotype as Genotype>::Allele>() as FitnessValue,
        )
    }
}

// /// placeholder for testing and bootstrapping, not really used in practice
// #[derive(Clone, Debug)]
// pub struct SumContinuousGenotype(pub SumContinuousGenotypePrecision);
// impl Fitness for SumContinuousGenotype {
//     type Genotype = ContinuousGenotype;
//     fn calculate_for_chromosome(
//         &mut self,
//         chromosome: &Chromosome<Self::Genotype>,
//     ) -> Option<FitnessValue> {
//         Some(
//             chromosome
//                 .genes
//                 .iter()
//                 .map(|v| v / self.0)
//                 .sum::<<Self::Genotype as Genotype>::Allele>() as FitnessValue,
//         )
//     }
// }

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
                .sum::<<Self::Genotype as Genotype>::Allele>() as FitnessValue,
        )
    }
}

pub type SumMultiContinuousGenotypePrecision = MultiContinuousGenotypeAllele;
/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumMultiContinuousGenotype(pub SumMultiContinuousGenotypePrecision);
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
                .map(|v| v / self.0)
                .sum::<<Self::Genotype as Genotype>::Allele>() as FitnessValue,
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
                .sum::<<Self::Genotype as Genotype>::Allele>() as FitnessValue,
        )
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumMultiUniqueGenotype;
impl Fitness for SumMultiUniqueGenotype {
    type Genotype = MultiUniqueGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Allele>() as FitnessValue,
        )
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct SumUniqueGenotype;
impl Fitness for SumUniqueGenotype {
    type Genotype = UniqueGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .sum::<<Self::Genotype as Genotype>::Allele>() as FitnessValue,
        )
    }
}
