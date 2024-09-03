//! The chromosome is a container for the genes and caches a fitness score
use crate::fitness::FitnessValue;
use crate::genotype::Genotype;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

/// The GenesKey can be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesKey = u64;

/// The Chromosome is used as an individual in the [Population](crate::population::Population). It
/// holds the genes and knows how to sort between itself with regard to it's fitness score.
/// Chromosomes [crossover](crate::crossover), [mutate](crate::mutate) and [compete](crate::compete) with each other in the
/// [Evolve](crate::strategy::evolve::Evolve) strategy
#[derive(Clone, Debug)]
pub struct Chromosome<G: Genotype> {
    pub genes: Vec<G::Allele>,
    pub fitness_score: Option<FitnessValue>,
    pub age: usize,

    /// User controlled alternative to `genes_key()`, set manually in
    /// custom [Fitness](crate::fitness::Fitness) implementation. Defaults to 0
    pub reference_id: usize,
}

/// Cannot Hash floats
impl<G: Genotype> Chromosome<G>
where
    G::Allele: Hash,
{
    pub fn genes_key(&self) -> GenesKey {
        let mut s = DefaultHasher::new();
        self.genes.hash(&mut s);
        s.finish()
    }
}

impl<G: Genotype> Chromosome<G> {
    pub fn new(genes: Vec<G::Allele>) -> Self {
        Self {
            genes,
            fitness_score: None,
            age: 0,
            reference_id: 0,
        }
    }
    /// Reset fitness_score for recalculation
    pub fn taint_fitness_score(&mut self) {
        self.age = 0;
        self.fitness_score = None;
    }
}

impl<G: Genotype> PartialEq for Chromosome<G> {
    fn eq(&self, other: &Self) -> bool {
        self.fitness_score == other.fitness_score
    }
}

impl<G: Genotype> Eq for Chromosome<G> {}

impl<G: Genotype> PartialOrd for Chromosome<G> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.fitness_score.cmp(&other.fitness_score))
    }
}

impl<G: Genotype> Ord for Chromosome<G> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<G: Genotype> fmt::Display for Chromosome<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(score) = self.fitness_score {
            write!(f, "fitness score {}", score)
        } else {
            write!(f, "no fitness score")
        }
    }
}
