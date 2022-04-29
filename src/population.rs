use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use stats::stddev;

#[derive(Debug, Clone)]
pub struct Population<T: Genotype> {
    pub chromosomes: Vec<Chromosome<T>>,
}

impl<T: Genotype> Population<T> {
    pub fn new(chromosomes: Vec<Chromosome<T>>) -> Self {
        Self {
            chromosomes: chromosomes,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            chromosomes: vec![],
        }
    }

    pub fn merge(&mut self, other: &mut Self) {
        self.chromosomes.append(&mut other.chromosomes);
    }

    pub fn sort(&mut self) {
        self.chromosomes.sort_unstable_by_key(|c| c.fitness_score);
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }

    pub fn best_chromosome(&self) -> Option<&Chromosome<T>> {
        self.chromosomes.iter().max()
    }

    pub fn fitness_score_stddev(&self) -> f32 {
        stddev(self.chromosomes.iter().filter_map(|c| c.fitness_score)) as f32
    }
}
