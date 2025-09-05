//! placeholders for testing and bootstrapping, not really used in practice
use crate::distributed::allele::RangeAllele;
use crate::distributed::chromosome::GenesOwner;
use crate::distributed::fitness::{Fitness, FitnessChromosome, FitnessPopulation, FitnessValue};
use crate::distributed::genotype::{
    BinaryGenotype, BitGenotype, DynamicRangeGenotype, Genotype, StaticRangeGenotype,
};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::marker::PhantomData;
use std::ops::Range;
use std::{thread, time};

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct Zero<G: Genotype>(PhantomData<G>);
impl<G: Genotype> Zero<G> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<G: Genotype> Default for Zero<G> {
    fn default() -> Self {
        Self::new()
    }
}
impl<G: Genotype> Fitness for Zero<G> {
    type Genotype = G;
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
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
        chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct CountOnes;
impl Fitness for CountOnes {
    type Genotype = BitGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.count_ones(..) as FitnessValue)
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
/// Sums the genes and converts to [FitnessValue]
/// There are 2 constructors:
/// * new(), precision is defaulted to 1.0
/// * new_with_precision(precision)
#[derive(Clone, Debug)]
pub struct SumGenes<G: Genotype> {
    precision: f64,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> SumGenes<G> {
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
impl<G: Genotype> Default for SumGenes<G> {
    fn default() -> Self {
        Self {
            precision: 1.0_f64,
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype> Fitness for SumGenes<G>
where
    G::Allele: Into<f64>,
    G::Genes: IntoIterator<Item = G::Allele>,
    G::Chromosome: GenesOwner<Genes = G::Genes>,
{
    type Genotype = G;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        let sum: f64 = chromosome
            .genes()
            .clone()
            .into_iter()
            .fold(0.0_f64, |acc, e| acc + e.into());
        Some((sum / self.precision) as FitnessValue)
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
/// Sums the dynamic matrix rows and converts to vector of [FitnessValue]
/// There are 2 constructors:
/// * new(), precision is defaulted to 1.0
/// * new_with_precision(precision)
#[derive(Clone, Debug)]
pub struct SumDynamicRange<T: RangeAllele + Into<f64>>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    precision: f64,
    _phantom: PhantomData<T>,
}
impl<T: RangeAllele + Into<f64>> SumDynamicRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
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
impl<T: RangeAllele + Into<f64>> Default for SumDynamicRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn default() -> Self {
        Self {
            precision: 1.0_f64,
            _phantom: PhantomData,
        }
    }
}
impl<T: RangeAllele + Into<f64>> Fitness for SumDynamicRange<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Genotype = DynamicRangeGenotype<T>;
    fn calculate_for_population(
        &mut self,
        _population: &FitnessPopulation<Self>,
        genotype: &Self::Genotype,
    ) -> Vec<Option<FitnessValue>> {
        genotype
            .data
            .chunks(genotype.genes_size())
            .map(|genes| {
                (genes.iter().copied().fold(0.0_f64, |acc, e| acc + e.into()) / self.precision)
                    as FitnessValue
            })
            .map(Some)
            .collect()
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
/// Sums the static matrix rows and converts to vector of [FitnessValue]
/// There are 2 constructors:
/// * new(), precision is defaulted to 1.0
/// * new_with_precision(precision)
#[derive(Clone, Debug)]
pub struct SumStaticRange<T: RangeAllele + Into<f64>, const N: usize, const M: usize>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    precision: f64,
    _phantom: PhantomData<T>,
}
impl<T: RangeAllele + Into<f64>, const N: usize, const M: usize> SumStaticRange<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
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
impl<T: RangeAllele + Into<f64>, const N: usize, const M: usize> Default
    for SumStaticRange<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn default() -> Self {
        Self {
            precision: 1.0_f64,
            _phantom: PhantomData,
        }
    }
}

impl<T: RangeAllele + Into<f64>, const N: usize, const M: usize> Fitness
    for SumStaticRange<T, N, M>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Genotype = StaticRangeGenotype<T, N, M>;
    fn calculate_for_population(
        &mut self,
        _population: &FitnessPopulation<Self>,
        genotype: &Self::Genotype,
    ) -> Vec<Option<FitnessValue>> {
        genotype
            .data
            .iter()
            .map(|genes| {
                (genes.iter().copied().fold(0.0_f64, |acc, e| acc + e.into()) / self.precision)
                    as FitnessValue
            })
            .map(Some)
            .collect()
    }
}

/// placeholder for testing and benchmarking, not used in practice
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
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
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

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct Countdown<G: Genotype>(usize, PhantomData<G>);
impl<G: Genotype> Countdown<G> {
    pub fn new(start: usize) -> Self {
        Self(start, PhantomData)
    }
}
impl<G: Genotype> Fitness for Countdown<G> {
    type Genotype = G;
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        if self.0 == 0 {
            Some(0)
        } else {
            self.0 -= 1;
            Some(self.0 as FitnessValue)
        }
    }
}

/// placeholder for testing and bootstrapping, not really used in practice
#[derive(Clone, Debug)]
pub struct CountdownNoisy<G: Genotype> {
    start: usize,
    step: usize,
    noise_sampler: Uniform<usize>,
    rng: SmallRng,
    _phantom: PhantomData<G>,
}
impl<G: Genotype> CountdownNoisy<G> {
    pub fn new(start: usize, step: usize, noise_range: Range<usize>) -> Self {
        Self {
            start,
            step,
            noise_sampler: Uniform::from(noise_range),
            rng: SmallRng::seed_from_u64(0),
            _phantom: PhantomData,
        }
    }
}
impl<G: Genotype> Fitness for CountdownNoisy<G> {
    type Genotype = G;
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
    ) -> Option<FitnessValue> {
        if self.start == 0 {
            Some(0)
        } else {
            self.start -= 1;
            let base = (self.start / self.step + 1) * self.step;
            let result = base + self.noise_sampler.sample(&mut self.rng);
            Some(result as FitnessValue)
        }
    }
}
