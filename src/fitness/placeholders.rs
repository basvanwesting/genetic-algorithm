//! placeholders for testing and bootstrapping, not really used in practice
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessValue};
use crate::genotype::{Allele, BinaryAllele};
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
/// Sums the genes and converts to isize [FitnessValue]
/// There are 2 constructors:
/// * new(), precision is defaulted to 1.0
/// * new_with_precision(precision)
#[derive(Clone, Debug)]
pub struct SumGenes<T: Allele + Into<f64>> {
    precision: f64,
    _phantom: PhantomData<T>,
}
impl<T: Allele + Into<f64>> SumGenes<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_with_precision(precision: f64) -> Self {
        Self {
            precision,
            ..Default::default()
        }
    }
}
impl<T: Allele + Into<f64>> Default for SumGenes<T> {
    fn default() -> Self {
        Self {
            precision: 1.0_f64,
            _phantom: PhantomData,
        }
    }
}
impl<T: Allele + Into<f64>> Fitness for SumGenes<T> {
    type Allele = T;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
    ) -> Option<FitnessValue> {
        let sum: f64 = chromosome
            .genes
            .clone()
            .into_iter()
            .fold(0.0_f64, |acc, e| acc + e.into());
        Some((sum / self.precision) as FitnessValue)
    }
}

/// placeholder for testing and benchmarking, not used in practice
use std::{thread, time};
#[derive(Debug)]
pub struct CountTrueWithSleep {
    pub micro_seconds: u64,
    pub print_on_clone: bool,
}
impl CountTrueWithSleep {
    pub fn new(micro_seconds: u64, print_on_clone: bool) -> Self {
        Self {
            micro_seconds,
            print_on_clone,
        }
    }
}
impl Fitness for CountTrueWithSleep {
    type Allele = BinaryAllele;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
    ) -> Option<FitnessValue> {
        thread::sleep(time::Duration::from_micros(self.micro_seconds));
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}
impl Clone for CountTrueWithSleep {
    fn clone(&self) -> Self {
        if self.print_on_clone {
            println!("Cloned CountTrueWithSleep: {:?}", thread::current().id());
        }
        Self {
            micro_seconds: self.micro_seconds,
            print_on_clone: self.print_on_clone,
        }
    }
}
