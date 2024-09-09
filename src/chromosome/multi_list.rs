use crate::fitness::FitnessValue;
use crate::genotype::{Allele, Genotype};
use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct MultiList<T: Allele> {
    pub genes: Vec<T>,
    pub fitness_score: Option<FitnessValue>,
    pub age: usize,
    pub reference_id: usize,
}

impl<T: Allele> MultiList<T> {
    pub fn new(genes: Vec<T>) -> Self {
        Self {
            genes,
            fitness_score: None,
            age: 0,
            reference_id: usize::MAX,
        }
    }
}

impl<T: Allele> super::Chromosome for MultiList<T> {
    fn age(&self) -> usize {
        self.age
    }
    fn reset_age(&mut self) {
        self.age = 0;
    }
    fn increment_age(&mut self) {
        self.age += 1
    }
    fn fitness_score(&self) -> Option<FitnessValue> {
        self.fitness_score
    }
    fn taint_fitness_score(&mut self) {
        self.age = 0;
        self.fitness_score = None;
    }
}

impl<T: Allele> MultiList<T>
where
    Vec<T>: Hash,
{
    pub fn genes_key(&self) -> super::GenesKey {
        let mut s = DefaultHasher::new();
        self.genes.hash(&mut s);
        s.finish()
    }
}
