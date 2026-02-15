//! The chromosome is a container for the genes and stores some useful values

use crate::allele::Allele;
use crate::fitness::FitnessValue;
use rustc_hash::FxHasher;
use std::hash::Hasher;

/// The GenesHash is used for determining cardinality in the population
/// It could also be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesHash = u64;

/// Type alias for genes - provides semantic clarity without trait overhead
/// Makes it clear when we're dealing with genes vs other vectors
pub type Genes<T> = Vec<T>;

/// The Chromosome is used as an individual in the [Population](crate::population::Population).
/// Chromosomes [select](crate::select), [crossover](crate::crossover) and [mutate](crate::mutate)
/// with each other in the [Evolve](crate::strategy::evolve::Evolve) strategy.
///
/// In the distributed module, chromosomes own their genes directly.
/// You can use [Evolve::best_chromosome()](crate::strategy::evolve::Evolve::best_chromosome),
/// [HillClimb::best_chromosome()](crate::strategy::hill_climb::HillClimb::best_chromosome) and
/// [Permutate::best_chromosome()](crate::strategy::permutate::Permutate::best_chromosome)
/// to access the best chromosome directly.
#[derive(Clone, Debug)]
pub struct Chromosome<T: Allele> {
    pub genes: Genes<T>,
    pub fitness_score: Option<FitnessValue>,
    pub genes_hash: Option<GenesHash>,
    pub age: usize,
}

impl<T: Allele> Chromosome<T> {
    pub fn new(genes: Genes<T>) -> Self {
        Self {
            genes,
            fitness_score: None,
            genes_hash: None,
            age: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            genes: Genes::with_capacity(capacity),
            fitness_score: None,
            genes_hash: None,
            age: 0,
        }
    }

    pub fn age(&self) -> usize {
        self.age
    }

    pub fn reset_age(&mut self) {
        self.age = 0;
    }

    pub fn increment_age(&mut self) {
        self.age += 1
    }

    pub fn set_age(&mut self, age: usize) {
        self.age = age
    }

    /// Returns true if age is 0 (newly created by crossover or initialization).
    pub fn is_offspring(&self) -> bool {
        self.age == 0
    }

    pub fn fitness_score(&self) -> Option<FitnessValue> {
        self.fitness_score
    }

    pub fn set_fitness_score(&mut self, fitness_score: Option<FitnessValue>) {
        self.fitness_score = fitness_score
    }

    pub fn genes_hash(&self) -> Option<GenesHash> {
        self.genes_hash
    }

    pub fn set_genes_hash(&mut self, genes_hash: Option<GenesHash>) {
        self.genes_hash = genes_hash
    }

    pub fn genes(&self) -> &Genes<T> {
        &self.genes
    }

    /// Reset age to 0, clear fitness score, and recalculate genes hash.
    /// Must be called after any direct gene manipulation (crossover, mutation).
    pub fn reset_metadata(&mut self, genes_hashing: bool) {
        self.age = 0;
        self.fitness_score = None;
        if genes_hashing {
            self.genes_hash = Some(self.calculate_hash())
        }
    }

    /// Copy age, fitness_score, and genes_hash from another chromosome.
    pub fn copy_metadata(&mut self, other: &Self) {
        self.age = other.age;
        self.fitness_score = other.fitness_score;
        self.genes_hash = other.genes_hash;
    }

    /// Copy genes and metadata from source. Used for chromosome recycling.
    pub fn copy_from(&mut self, source: &Self) {
        // For recycled chromosomes, this is just memcpy with known size
        self.genes.clone_from(&source.genes);
        self.copy_metadata(source);
    }

    pub fn calculate_hash(&self) -> GenesHash {
        let mut hasher = FxHasher::default();
        T::hash_slice(&self.genes, &mut hasher);
        hasher.finish()
    }
}
