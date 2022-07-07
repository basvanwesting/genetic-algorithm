//! The population is a  container for [Chromosomes](Chromosome)
use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use crate::genotype::Genotype;

#[derive(Clone, Debug)]
pub struct Population<T: Genotype> {
    pub chromosomes: Vec<Chromosome<T>>,
}

impl<T: Genotype> Population<T> {
    pub fn new(chromosomes: Vec<Chromosome<T>>) -> Self {
        Self { chromosomes }
    }

    pub fn new_empty() -> Self {
        Self {
            chromosomes: vec![],
        }
    }

    pub fn merge(&mut self, other: &mut Self) {
        self.chromosomes.append(&mut other.chromosomes);
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }

    /// fitness_score is Option and None is least, but invalid as best_chromosome, so filter it out
    /// when minimizing the fitness score, otherwise None would end up as best.
    pub fn best_chromosome(&self, fitness_ordering: FitnessOrdering) -> Option<&Chromosome<T>> {
        match fitness_ordering {
            FitnessOrdering::Maximize => self.chromosomes.iter().max(),
            FitnessOrdering::Minimize => self
                .chromosomes
                .iter()
                .filter(|c| c.fitness_score.is_some())
                .min(),
        }
    }

    pub fn fitness_score_stddev(&self) -> f32 {
        stats::stddev(self.chromosomes.iter().filter_map(|c| c.fitness_score)) as f32
    }
    pub fn fitness_score_median(&self) -> Option<isize> {
        stats::median(self.chromosomes.iter().filter_map(|c| c.fitness_score)).map(|v| v as isize)
    }
    pub fn fitness_score_mean(&self) -> f32 {
        stats::mean(self.chromosomes.iter().filter_map(|c| c.fitness_score)) as f32
    }

    pub fn fitness_score_count(&self) -> usize {
        self.chromosomes
            .iter()
            .filter(|c| c.fitness_score.is_some())
            .count()
    }
}

impl<T: Genotype> From<Vec<Chromosome<T>>> for Population<T> {
    fn from(chromosomes: Vec<Chromosome<T>>) -> Self {
        Self::new(chromosomes)
    }
}
