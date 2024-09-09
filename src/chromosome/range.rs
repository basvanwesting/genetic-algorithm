use crate::fitness::FitnessValue;
use crate::genotype::{Allele};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Range<T: Allele> {
    pub genes: Vec<T>,
    pub fitness_score: Option<FitnessValue>,
    pub age: usize,
    pub reference_id: usize,
}

impl<T: Allele> Range<T> {
    pub fn new(genes: Vec<T>) -> Self {
        Self {
            genes,
            fitness_score: None,
            age: 0,
            reference_id: usize::MAX,
        }
    }
}

impl<T: Allele> super::Chromosome for Range<T> {
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
impl<T: Allele> super::OwnesGenes for Range<T> {
    type Genes = Vec<T>;
    fn genes(&self) -> &Vec<T> {
        &self.genes
    }
}

impl<T: Allele> Range<T>
where
    Vec<T>: Hash,
{
    pub fn genes_key(&self) -> super::GenesKey {
        let mut s = DefaultHasher::new();
        self.genes.hash(&mut s);
        s.finish()
    }
}
