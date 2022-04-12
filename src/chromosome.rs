use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Chromosome {
    pub genes: Vec<bool>,
    pub fitness: Option<usize>,
}

impl Chromosome {
    pub fn new(genes: Vec<bool>) -> Self {
        Self {
            genes: genes,
            fitness: None,
        }
    }
}

impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for Chromosome {}

impl PartialOrd for Chromosome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.fitness.cmp(&other.fitness))
    }
}

impl Ord for Chromosome {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl fmt::Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(score) = self.fitness {
            write!(f, "fitness score {}", score)
        } else {
            write!(f, "no fitness score")
        }
    }
}
