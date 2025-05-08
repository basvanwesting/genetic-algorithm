//! The population is a  container for [Chromosomes](Chromosome)
use crate::chromosome::Chromosome;
use crate::fitness::{FitnessOrdering, FitnessValue};
use cardinality_estimator::CardinalityEstimator;
use rand::prelude::*;
use std::cmp::Reverse;

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

    // Returns one less than total size with known fitness due to implementation constraints
    // Doesn't matter the amount should be much less than the population size
    pub fn best_chromosome_indices(
        &self,
        amount: usize,
        fitness_ordering: FitnessOrdering,
    ) -> Vec<usize> {
        let mut data: Vec<(usize, isize)> = self
            .chromosomes
            .iter()
            .filter_map(|c| c.fitness_score())
            .enumerate()
            .collect();

        if data.is_empty() {
            Vec::new()
        } else {
            let index = amount.min(data.len().saturating_sub(1));
            let (lesser, _median, _greater) = match fitness_ordering {
                FitnessOrdering::Maximize => {
                    data.select_nth_unstable_by_key(index, |(_, score)| Reverse(*score))
                }
                FitnessOrdering::Minimize => {
                    data.select_nth_unstable_by_key(index, |(_, score)| *score)
                }
            };
            let mut result: Vec<usize> = lesser.iter().map(|(idx, _)| *idx).collect();
            result.sort_unstable();
            result
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
    pub fn fitness_score_cardinality(&self) -> Option<usize> {
        let mut values = self
            .chromosomes
            .iter()
            .filter_map(|c| c.fitness_score())
            .peekable();
        if values.peek().is_some() {
            let mut estimator = CardinalityEstimator::<FitnessValue>::new();
            values.for_each(|fitness_score| estimator.insert(&fitness_score));
            Some(estimator.estimate())
        } else {
            None
        }
    }
    pub fn genes_cardinality(&self) -> Option<usize> {
        let mut values = self
            .chromosomes
            .iter()
            .filter_map(|c| c.genes_hash())
            .peekable();
        if values.peek().is_some() {
            let mut estimator = CardinalityEstimator::<u64>::new();
            values.for_each(|genes_hash| estimator.insert_hash(genes_hash));
            Some(estimator.estimate())
        } else {
            None
        }
    }
}

impl<C: Chromosome> From<Vec<C>> for Population<C> {
    fn from(chromosomes: Vec<C>) -> Self {
        Self::new(chromosomes)
    }
}
