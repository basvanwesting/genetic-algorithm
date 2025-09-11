use super::GenesHash;
use crate::centralized::fitness::FitnessValue;

#[derive(Copy, Clone, Debug)]
pub struct Chromosome {
    pub row_id: usize,
    pub fitness_score: Option<FitnessValue>,
    pub genes_hash: Option<GenesHash>,
    pub age: usize,
}

impl Chromosome {
    pub fn new(row_id: usize) -> Self {
        Self {
            row_id,
            fitness_score: None,
            genes_hash: None,
            age: 0,
        }
    }

    pub fn age(&self) -> usize {
        self.age
    }

    pub fn reset_age(&mut self) {
        self.age = 0;
    }

    pub fn increment_age(&mut self) {
        self.age += 1
    }

    pub fn set_age(&mut self, age: usize) {
        self.age = age
    }

    pub fn is_offspring(&self) -> bool {
        self.age == 0
    }

    pub fn fitness_score(&self) -> Option<FitnessValue> {
        self.fitness_score
    }

    pub fn set_fitness_score(&mut self, fitness_score: Option<FitnessValue>) {
        self.fitness_score = fitness_score
    }

    pub fn genes_hash(&self) -> Option<GenesHash> {
        self.genes_hash
    }

    pub fn set_genes_hash(&mut self, genes_hash: Option<GenesHash>) {
        self.genes_hash = genes_hash
    }

    pub fn reset_state(&mut self, genes_hash: Option<GenesHash>) {
        self.age = 0;
        self.fitness_score = None;
        self.genes_hash = genes_hash;
    }

    pub fn copy_state(&mut self, other: &Self) {
        self.age = other.age;
        self.fitness_score = other.fitness_score;
        self.genes_hash = other.genes_hash;
    }
}
