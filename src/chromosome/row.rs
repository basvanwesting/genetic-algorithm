use super::GenesHash;
use crate::fitness::FitnessValue;

#[derive(Copy, Clone, Debug)]
pub struct Row {
    pub row_id: usize,
    pub fitness_score: Option<FitnessValue>,
    pub genes_hash: Option<GenesHash>,
    pub age: usize,
    pub reference_id: usize,
}

impl super::Chromosome for Row {
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
    fn genes_hash(&self) -> Option<GenesHash> {
        self.genes_hash
    }
    fn set_genes_hash(&mut self, genes_hash: Option<GenesHash>) {
        self.genes_hash = genes_hash
    }
    fn taint(&mut self, genes_hash: GenesHash) {
        self.age = 0;
        self.fitness_score = None;
        self.genes_hash = Some(genes_hash);
        self.reference_id = usize::MAX;
    }
    fn copy_fields_from(&mut self, other: &Self) {
        self.age = other.age;
        self.fitness_score = other.fitness_score;
        self.genes_hash = other.genes_hash;
        self.reference_id = other.reference_id;
    }
}
impl super::GenesPointer for Row {
    fn new(row_id: usize) -> Self {
        Self {
            row_id,
            fitness_score: None,
            genes_hash: None,
            age: 0,
            reference_id: usize::MAX,
        }
    }
}
