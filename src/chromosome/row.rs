use crate::fitness::FitnessValue;

#[derive(Clone, Debug)]
pub struct Row {
    pub row_id: usize,
    pub fitness_score: Option<FitnessValue>,
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
    fn taint(&mut self) {
        self.age = 0;
        self.fitness_score = None;
        self.reference_id = usize::MAX;
    }
    fn copy_fields_from(&mut self, other: &Self) {
        self.age = other.age;
        self.fitness_score = other.fitness_score;
        self.reference_id = other.reference_id;
    }
}
impl super::RefersGenes for Row {
    fn new(row_id: usize) -> Self {
        Self {
            row_id,
            fitness_score: None,
            age: 0,
            reference_id: usize::MAX,
        }
    }
}
