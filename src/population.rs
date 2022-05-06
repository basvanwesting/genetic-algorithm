use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use crate::genotype::Genotype;
use stats::stddev;

#[derive(Debug)]
pub struct Population<T: Genotype> {
    pub chromosomes: Vec<Chromosome<T>>,
}

impl<T: Genotype> Population<T> {
    pub fn new(chromosomes: Vec<Chromosome<T>>) -> Self {
        Self { chromosomes }
    }

    pub fn new_empty() -> Self {
        Self {
            chromosomes: vec![],
        }
    }

    pub fn merge(&mut self, other: &mut Self) {
        self.chromosomes.append(&mut other.chromosomes);
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }

    // fitness_score is Option and None is least, but invalid as best_chromosome, so filter it out
    pub fn best_chromosome(&self, fitness_ordering: FitnessOrdering) -> Option<&Chromosome<T>> {
        match fitness_ordering {
            FitnessOrdering::Maximize => self.chromosomes.iter().max(),
            FitnessOrdering::Minimize => self
                .chromosomes
                .iter()
                .filter(|c| c.fitness_score.is_some())
                .min(),
        }
    }

    pub fn fitness_score_stddev(&self) -> f32 {
        stddev(self.chromosomes.iter().filter_map(|c| c.fitness_score)) as f32
    }
}

// manually implement Clone, because derive requires Genotype to be Clone as well
impl<T: Genotype> Clone for Population<T> {
    fn clone(&self) -> Population<T> {
        Self {
            chromosomes: self.chromosomes.clone(),
        }
    }
}
