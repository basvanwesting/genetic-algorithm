//! The population is a  container for [Chromosomes](Chromosome)
use crate::allele::Allele;
use crate::chromosome::{Chromosome, GenesHash};
use crate::fitness::{FitnessOrdering, FitnessValue};
use cardinality_estimator::CardinalityEstimator;
use itertools::Itertools;
use rand::prelude::*;
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Population<T: Allele> {
    pub chromosomes: Vec<Chromosome<T>>,
    pub recycling: bool,
    recycling_bin: Vec<Chromosome<T>>,
}

impl<T: Allele> Population<T> {
    pub fn new(chromosomes: Vec<Chromosome<T>>, recycling: bool) -> Self {
        Self {
            chromosomes,
            recycling,
            recycling_bin: Vec::new(),
        }
    }

    pub fn new_empty(recycling: bool) -> Self {
        Self {
            chromosomes: vec![],
            recycling,
            recycling_bin: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }
    pub fn parents_and_offspring_size(&self) -> (usize, usize) {
        self.chromosomes
            .iter()
            .fold((0, 0), |(parents_size, offspring_size), chromosome| {
                if chromosome.is_offspring() {
                    (parents_size, offspring_size + 1)
                } else {
                    (parents_size + 1, offspring_size)
                }
            })
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

    pub fn recycled_size(&self) -> usize {
        self.recycling_bin.len()
    }

    /// Get a recycled chromosome or create new one by cloning source
    pub fn new_chromosome(&mut self, source: &Chromosome<T>) -> Chromosome<T> {
        if self.recycling {
            if let Some(mut recycled) = self.recycling_bin.pop() {
                recycled.copy_from(source);
                recycled
            } else {
                source.clone()
            }
        } else {
            source.clone()
        }
    }

    /// Recycle the chromosome or just drop it
    pub fn drop_chromosome(&mut self, chromosome: Chromosome<T>) {
        if self.recycling {
            self.recycling_bin.push(chromosome)
        } else {
            // just let go out of scope
        }
    }

    /// Truncate a population and add truncated chromosomes to recycling bin
    pub fn truncate(&mut self, keep_size: usize) {
        if self.recycling {
            self.chromosomes
                .drain(keep_size..)
                .for_each(|c| self.recycling_bin.push(c));
        } else {
            self.chromosomes.truncate(keep_size);
        }
    }

    /// Truncate a detached vector and add truncated chromosomes to recycling bin
    /// Used when chromosomes are temporarily outside the population (e.g. during selection)
    pub fn truncate_external(&mut self, chromosomes: &mut Vec<Chromosome<T>>, keep_size: usize) {
        if self.recycling {
            chromosomes
                .drain(keep_size..)
                .for_each(|c| self.recycling_bin.push(c));
        } else {
            chromosomes.truncate(keep_size);
        }
    }

    /// Extend population by amount, reusing recycled chromosomes if available
    pub fn extend_from_within(&mut self, amount: usize) {
        if self.recycling {
            for i in 0..amount {
                let source = &self.chromosomes[i];
                let chromosome = if let Some(mut recycled) = self.recycling_bin.pop() {
                    recycled.copy_from(source);
                    recycled
                } else {
                    source.clone()
                };
                self.chromosomes.push(chromosome);
            }
        } else {
            self.chromosomes.extend_from_within(0..amount);
        }
    }

    /// fitness_score is Option and None is least, but invalid as best_chromosome, so filter it out
    /// when minimizing the fitness score, otherwise None would end up as best.
    pub fn best_chromosome(&self, fitness_ordering: FitnessOrdering) -> Option<&Chromosome<T>> {
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

    // In summary a bit quirky, but fast and doesn't require genes_hashing,
    // which doesn't matter as the amount should be much less than the population size (usage in elitism_rate)
    //
    // Returns one less than total size with known fitness due to implementation constraints.
    // Does not care about uniqueness of the genes_hash.
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

    // Only works when genes_hash is stored on chromosome, as this is the uniqueness key.
    // Takes the first index occurence of a genes_hash. Returns indices in ascending order
    pub fn unique_chromosome_indices(&self) -> Vec<usize> {
        let mut data: HashMap<GenesHash, usize> = HashMap::new();
        self.chromosomes
            .iter()
            .enumerate()
            .for_each(|(index, chromosome)| {
                if let Some(genes_hash) = chromosome.genes_hash() {
                    data.entry(genes_hash).or_insert_with(|| index);
                }
            });
        data.into_values().sorted().collect()
    }

    // Only works when genes_hash is stored on chromosome, as this is the uniqueness key.
    // Takes the first index occurence of a genes_hash. Returns indices in ascending order (irrespective of fitness)
    pub fn best_unique_chromosome_indices(
        &self,
        amount: usize,
        fitness_ordering: FitnessOrdering,
    ) -> Vec<usize> {
        let mut data: HashMap<GenesHash, (usize, isize)> = HashMap::new();
        self.chromosomes
            .iter()
            .enumerate()
            .for_each(|(index, chromosome)| {
                if let Some(genes_hash) = chromosome.genes_hash() {
                    if let Some(fitness_score) = chromosome.fitness_score() {
                        data.entry(genes_hash)
                            .or_insert_with(|| (index, fitness_score));
                    }
                }
            });

        if data.is_empty() {
            Vec::new()
        } else {
            let iterator = match fitness_ordering {
                FitnessOrdering::Maximize => data
                    .into_values()
                    .sorted_unstable_by_key(|(_, score)| Reverse(*score)),
                FitnessOrdering::Minimize => data
                    .into_values()
                    .sorted_unstable_by_key(|(_, score)| *score),
            };
            iterator.take(amount).map(|(idx, _)| idx).sorted().collect()
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

// FIXME: still used?
impl<T: Allele> From<Vec<Chromosome<T>>> for Population<T> {
    fn from(chromosomes: Vec<Chromosome<T>>) -> Self {
        Self::new(chromosomes, false)
    }
}
