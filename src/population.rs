//! The population is a  container for [Chromosomes](Chromosome)
use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use crate::genotype::Genotype;
use crate::strategy::evolve::EvolveConfig;
use cardinality_estimator::CardinalityEstimator;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Population<G: Genotype> {
    pub chromosomes: Vec<Chromosome<G>>,
}

impl<G: Genotype> Population<G> {
    pub fn new(chromosomes: Vec<Chromosome<G>>) -> Self {
        Self { chromosomes }
    }

    pub fn new_empty() -> Self {
        Self {
            chromosomes: vec![],
        }
    }

    // pub fn merge(&mut self, other: &mut Self) {
    //     self.chromosomes.append(&mut other.chromosomes);
    // }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }
    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        self.chromosomes.shuffle(rng);
    }

    pub fn reset_age(&mut self) {
        self.chromosomes.iter_mut().for_each(|c| c.age = 0);
    }
    pub fn increment_age(&mut self) {
        self.chromosomes.iter_mut().for_each(|c| c.age += 1);
    }

    /// fitness_score is Option and None is least, but invalid as best_chromosome, so filter it out
    /// when minimizing the fitness score, otherwise None would end up as best.
    pub fn best_chromosome(&self, fitness_ordering: FitnessOrdering) -> Option<&Chromosome<G>> {
        match fitness_ordering {
            FitnessOrdering::Maximize => self.chromosomes.iter().max(),
            FitnessOrdering::Minimize => self
                .chromosomes
                .iter()
                .filter(|c| c.fitness_score.is_some())
                .min(),
        }
    }

    pub fn age_mean(&self) -> f32 {
        stats::mean(self.chromosomes.iter().map(|c| c.age)) as f32
    }
    pub fn fitness_score_count(&self) -> usize {
        self.chromosomes
            .iter()
            .filter(|c| c.fitness_score.is_some())
            .count()
    }
    pub fn fitness_score_median(&self) -> Option<isize> {
        stats::median(self.chromosomes.iter().filter_map(|c| c.fitness_score)).map(|v| v as isize)
    }
    pub fn fitness_score_mean(&self) -> f32 {
        stats::mean(self.chromosomes.iter().filter_map(|c| c.fitness_score)) as f32
    }
    pub fn fitness_score_stddev(&self) -> f32 {
        stats::stddev(self.chromosomes.iter().filter_map(|c| c.fitness_score)) as f32
    }
    pub fn fitness_score_cardinality(&self) -> usize {
        let mut estimator = CardinalityEstimator::<isize>::new();
        let mut nones = 0;
        self.chromosomes.iter().for_each(|chromosome| {
            if let Some(fitness_score) = chromosome.fitness_score {
                estimator.insert(&fitness_score);
            } else {
                nones += 1;
            }
        });
        estimator.estimate() + nones
    }
    pub fn fitness_score_present(&self, fitness_score: Option<isize>) -> bool {
        self.chromosomes
            .iter()
            .any(|c| c.fitness_score == fitness_score)
    }
}

impl<G: Genotype> From<Vec<Chromosome<G>>> for Population<G> {
    fn from(chromosomes: Vec<Chromosome<G>>) -> Self {
        Self::new(chromosomes)
    }
}
