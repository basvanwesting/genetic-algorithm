use crate::chromosome::Chromosome;
use crate::gene::{BinaryGene, ContinuousGene, DiscreteGene, Gene};
use crate::genotype::Genotype;
use crate::population::Population;

pub trait Fitness<T: Gene>: Clone + std::fmt::Debug {
    fn call_for_population<G: Genotype<Gene = T>>(
        &self,
        mut population: Population<G>,
    ) -> Population<G> {
        population
            .chromosomes
            .iter_mut()
            .filter(|c| c.fitness_score.is_none())
            .for_each(|c| c.fitness_score = Some(self.call_for_chromosome(c)));
        population
    }
    fn call_for_chromosome<G: Genotype<Gene = T>>(&self, chromosome: &Chromosome<G>) -> isize;
}

#[derive(Clone, Debug)]
pub struct SimpleSum;
impl Fitness<BinaryGene> for SimpleSum {
    fn call_for_chromosome<G: Genotype<Gene = BinaryGene>>(
        &self,
        chromosome: &Chromosome<G>,
    ) -> isize {
        chromosome.genes.iter().filter(|&value| *value).count() as isize
    }
}

impl Fitness<DiscreteGene> for SimpleSum {
    fn call_for_chromosome<G: Genotype<Gene = DiscreteGene>>(
        &self,
        chromosome: &Chromosome<G>,
    ) -> isize {
        chromosome.genes.iter().sum::<DiscreteGene>() as isize
    }
}

impl Fitness<ContinuousGene> for SimpleSum {
    fn call_for_chromosome<G: Genotype<Gene = ContinuousGene>>(
        &self,
        chromosome: &Chromosome<G>,
    ) -> isize {
        chromosome.genes.iter().sum::<ContinuousGene>() as isize
    }
}
