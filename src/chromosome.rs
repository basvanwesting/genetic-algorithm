use crate::genotype::Genotype;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

pub type GenesKey = u64;

#[derive(Debug)]
pub struct Chromosome<T: Genotype> {
    pub genes: Vec<T::Gene>,
    pub fitness_score: Option<isize>,
}

// Cannot Hash ContinuousGene
impl<T: Genotype> Chromosome<T>
where
    <T as Genotype>::Gene: Hash,
{
    // genes_key() can be used for caching, without lifetime concerns of the chromosome
    pub fn genes_key(&self) -> GenesKey {
        let mut s = DefaultHasher::new();
        self.genes.hash(&mut s);
        s.finish()
    }
}

impl<T: Genotype> Chromosome<T> {
    pub fn new(genes: Vec<T::Gene>) -> Self {
        Self {
            genes,
            fitness_score: None,
        }
    }

    pub fn taint_fitness_score(&mut self) {
        self.fitness_score = None;
    }
}

// manually implement Clone, because derive requires Genotype to be Clone as well
impl<T: Genotype> Clone for Chromosome<T> {
    fn clone(&self) -> Chromosome<T> {
        Self {
            genes: self.genes.clone(),
            fitness_score: self.fitness_score,
        }
    }
}

impl<T: Genotype> PartialEq for Chromosome<T> {
    fn eq(&self, other: &Self) -> bool {
        self.fitness_score == other.fitness_score
    }
}

impl<T: Genotype> Eq for Chromosome<T> {}

impl<T: Genotype> PartialOrd for Chromosome<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.fitness_score.cmp(&other.fitness_score))
    }
}

impl<T: Genotype> Ord for Chromosome<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<T: Genotype> fmt::Display for Chromosome<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(score) = self.fitness_score {
            write!(f, "fitness score {}", score)
        } else {
            write!(f, "no fitness score")
        }
    }
}
