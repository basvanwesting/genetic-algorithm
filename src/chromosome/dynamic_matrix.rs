use crate::fitness::FitnessValue;
use crate::genotype::{Allele, Genotype};
use rand::prelude::*;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct DynamicMatrix {
    pub row_id: usize,
    pub fitness_score: Option<FitnessValue>,
    pub age: usize,
    pub reference_id: usize,
}

impl DynamicMatrix {
    pub fn new(row_id: usize) -> Self {
        Self {
            row_id,
            fitness_score: None,
            age: 0,
            reference_id: usize::MAX,
        }
    }
}

impl super::Chromosome for DynamicMatrix {
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
