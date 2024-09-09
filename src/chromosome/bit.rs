use crate::fitness::FitnessValue;
use fixedbitset::{FixedBitSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Bit {
    pub genes: FixedBitSet,
    pub fitness_score: Option<FitnessValue>,
    pub age: usize,
    pub reference_id: usize,
}

impl Bit {
    pub fn new(genes: FixedBitSet) -> Self {
        Self {
            genes,
            fitness_score: None,
            age: 0,
            reference_id: usize::MAX,
        }
    }
}

impl super::Chromosome for Bit {
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
    fn set_fitness_score(&mut self, fitness_score: Option<FitnessValue>) {
        self.fitness_score = fitness_score
    }
    fn taint_fitness_score(&mut self) {
        self.age = 0;
        self.fitness_score = None;
    }
}
impl super::OwnesGenes for Bit {
    type Genes = FixedBitSet;
    fn genes(&self) -> &FixedBitSet {
        &self.genes
    }
}

impl Bit {
    pub fn genes_key(&self) -> super::GenesKey {
        let mut s = DefaultHasher::new();
        self.genes.hash(&mut s);
        s.finish()
    }
}
