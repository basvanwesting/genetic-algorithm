use crate::gene::Gene;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Chromosome<T: Gene> {
    pub genes: Vec<T>,
    pub fitness_score: Option<usize>,
}

impl<T: Gene> Chromosome<T> {
    pub fn new(genes: Vec<T>) -> Self {
        Self {
            genes: genes,
            fitness_score: None,
        }
    }
}

impl<T: Gene> PartialEq for Chromosome<T> {
    fn eq(&self, other: &Self) -> bool {
        self.fitness_score == other.fitness_score
    }
}

impl<T: Gene> Eq for Chromosome<T> {}

impl<T: Gene> PartialOrd for Chromosome<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.fitness_score.cmp(&other.fitness_score))
    }
}

impl<T: Gene> Ord for Chromosome<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<T: Gene> fmt::Display for Chromosome<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(score) = self.fitness_score {
            write!(f, "fitness score {}", score)
        } else {
            write!(f, "no fitness score")
        }
    }
}
