//! The population is a  container for [Chromosomes](Chromosome)
use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use cardinality_estimator::CardinalityEstimator;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct Population<C: Chromosome> {
    pub chromosomes: Vec<C>,
}

impl<C: Chromosome> Population<C> {
    pub fn new(chromosomes: Vec<C>) -> Self {
        Self { chromosomes }
    }

    pub fn new_empty() -> Self {
        Self {
            chromosomes: vec![],
        }
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }
    pub fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        self.chromosomes.shuffle(rng);
    }

    pub fn reset_age(&mut self) {
        self.chromosomes.iter_mut().for_each(|c| c.reset_age())
    }
    pub fn increment_age(&mut self) {
        self.chromosomes.iter_mut().for_each(|c| c.increment_age())
    }

    /// fitness_score is Option and None is least, but invalid as best_chromosome, so filter it out
    /// when minimizing the fitness score, otherwise None would end up as best.
    pub fn best_chromosome(&self, fitness_ordering: FitnessOrdering) -> Option<&C> {
        if let Some(index) = self.best_chromosome_index(fitness_ordering) {
            self.chromosomes.get(index)
        } else {
            None
        }
    }
    pub fn best_chromosome_index(&self, fitness_ordering: FitnessOrdering) -> Option<usize> {
        match fitness_ordering {
            FitnessOrdering::Maximize => self
                .chromosomes
                .iter()
                .enumerate()
                .max_by_key(|(_idx, c)| c.fitness_score())
                .map(|(idx, _)| idx),

            FitnessOrdering::Minimize => self
                .chromosomes
                .iter()
                .filter(|c| c.fitness_score().is_some())
                .enumerate()
                .min_by_key(|(_idx, c)| c.fitness_score())
                .map(|(idx, _)| idx),
        }
    }

    pub fn age_mean(&self) -> f32 {
        stats::mean(self.chromosomes.iter().map(|c| c.age())) as f32
    }
    pub fn fitness_score_count(&self) -> usize {
        self.chromosomes
            .iter()
            .filter(|c| c.fitness_score().is_some())
            .count()
    }
    pub fn fitness_score_median(&self) -> Option<isize> {
        stats::median(self.chromosomes.iter().filter_map(|c| c.fitness_score())).map(|v| v as isize)
    }
    pub fn fitness_score_mean(&self) -> f32 {
        stats::mean(self.chromosomes.iter().filter_map(|c| c.fitness_score())) as f32
    }
    pub fn fitness_score_stddev(&self) -> f32 {
        stats::stddev(self.chromosomes.iter().filter_map(|c| c.fitness_score())) as f32
    }
    pub fn fitness_score_cardinality(&self) -> usize {
        let mut estimator = CardinalityEstimator::<isize>::new();
        let mut nones = 0;
        self.chromosomes.iter().for_each(|chromosome| {
            if let Some(fitness_score) = chromosome.fitness_score() {
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
            .any(|c| c.fitness_score() == fitness_score)
    }
    pub fn genes_cardinality(&self) -> usize {
        let mut estimator = CardinalityEstimator::<u64>::new();
        self.chromosomes.iter().for_each(|chromosome| {
            if let Some(genes_hash) = chromosome.genes_hash() {
                estimator.insert(&genes_hash);
            }
        });
        estimator.estimate()
    }
}

impl<C: Chromosome> From<Vec<C>> for Population<C> {
    fn from(chromosomes: Vec<C>) -> Self {
        Self::new(chromosomes)
    }
}
