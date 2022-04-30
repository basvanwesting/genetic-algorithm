use crate::chromosome::Chromosome;
use crate::gene::{ContinuousGene, DiscreteGene};
use crate::genotype::{
    BinaryGenotype, ContinuousGenotype, DiscreteGenotype, Genotype, RangeGenotype,
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
    fn call_for_chromosome(&self, chromosome: &Chromosome<BinaryGenotype>) -> isize {
        chromosome.genes.iter().filter(|&value| *value).count() as isize
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumDiscreteGenotypeDiscreteGene;
impl Fitness for SimpleSumDiscreteGenotypeDiscreteGene {
    type Genotype = DiscreteGenotype<DiscreteGene>;
    fn call_for_chromosome(
        &self,
        chromosome: &Chromosome<DiscreteGenotype<DiscreteGene>>,
    ) -> isize {
        chromosome.genes.iter().sum::<DiscreteGene>() as isize
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumDiscreteGenotypeContinuousGene;
impl Fitness for SimpleSumDiscreteGenotypeContinuousGene {
    type Genotype = DiscreteGenotype<ContinuousGene>;
    fn call_for_chromosome(
        &self,
        chromosome: &Chromosome<DiscreteGenotype<ContinuousGene>>,
    ) -> isize {
        chromosome.genes.iter().sum::<ContinuousGene>() as isize
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumContinuousGenotype;
impl Fitness for SimpleSumContinuousGenotype {
    type Genotype = ContinuousGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<ContinuousGenotype>) -> isize {
        chromosome.genes.iter().sum::<ContinuousGene>() as isize
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumRangeGenotypeDiscreteGene;
impl Fitness for SimpleSumRangeGenotypeDiscreteGene {
    type Genotype = RangeGenotype<DiscreteGene>;
    fn call_for_chromosome(&self, chromosome: &Chromosome<RangeGenotype<DiscreteGene>>) -> isize {
        chromosome.genes.iter().sum::<DiscreteGene>() as isize
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSumRangeGenotypeContinuousGene;
impl Fitness for SimpleSumRangeGenotypeContinuousGene {
    type Genotype = RangeGenotype<ContinuousGene>;
    fn call_for_chromosome(&self, chromosome: &Chromosome<RangeGenotype<ContinuousGene>>) -> isize {
        chromosome.genes.iter().sum::<ContinuousGene>() as isize
    }
}
