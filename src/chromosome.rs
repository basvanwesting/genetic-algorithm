use crate::gene::{Gene, GeneTrait};
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Chromosome<T: GeneTrait> {
    pub genes: Vec<Gene<T>>,
    pub fitness: Option<usize>,
}

impl<T: GeneTrait> Chromosome<T> {
    pub fn new(genes: Vec<Gene<T>>) -> Self {
        Self {
            genes: genes,
            fitness: None,
        }
    }
}

impl<T: GeneTrait> PartialEq for Chromosome<T> {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl<T: GeneTrait> Eq for Chromosome<T> {}

impl<T: GeneTrait> PartialOrd for Chromosome<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.fitness.cmp(&other.fitness))
    }
}

impl<T: GeneTrait> Ord for Chromosome<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<T: GeneTrait> fmt::Display for Chromosome<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(score) = self.fitness {
            write!(f, "fitness score {}", score)
        } else {
            write!(f, "no fitness score")
        }
    }
}
