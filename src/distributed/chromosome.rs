//! The chromosome is a container for the genes and stores some useful values

use crate::distributed::allele::Allele;
use crate::distributed::fitness::FitnessValue;
use crate::distributed::genotype::Genotype;
use rand::prelude::*;
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

    pub fn get_genes(&self) -> Genes<T> {
        self.genes.clone()
    }

    pub fn set_genes(&mut self, genes: Genes<T>) {
        self.genes = genes;
        let hash = self.calculate_hash();
        self.genes_hash = Some(hash);
        self.fitness_score = None;
        self.age = 0;
    }

    pub fn update_state(&mut self) {
        self.age = 0;
        self.fitness_score = None;
        self.genes_hash = Some(self.calculate_hash());
    }

    pub fn reset_state(&mut self, genes_hash: Option<GenesHash>) {
        self.age = 0;
        self.fitness_score = None;
        self.genes_hash = genes_hash;
    }

    pub fn copy_state(&mut self, other: &Self) {
        self.age = other.age;
        self.fitness_score = other.fitness_score;
        self.genes_hash = other.genes_hash;
    }

    pub fn copy_from(&mut self, source: &Self) {
        self.genes.clone_from(&source.genes);
        self.copy_state(source);
    }

    /// Calculate hash from genes using type-specific hashing
    pub fn calculate_hash(&self) -> GenesHash {
        let mut hasher = FxHasher::default();
        T::hash_slice(&self.genes, &mut hasher);
        hasher.finish()
    }
}

pub trait ChromosomeManager<G: Genotype> {
    /// Create random genes based on genotype configuration
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Genes<G::Allele>;
    /// Get the capacity hint for creating new chromosomes
    fn genes_capacity(&self) -> usize;

    // Helper methods using the new chromosome capabilities
    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut Chromosome<G::Allele>, rng: &mut R) {
        let genes = self.random_genes_factory(rng);
        chromosome.set_genes(genes);
    }

    fn chromosome_constructor_genes(&mut self, genes: &Genes<G::Allele>) -> Chromosome<G::Allele> {
        Chromosome::new(genes.clone())
    }

    fn chromosome_constructor_random<R: Rng>(&mut self, rng: &mut R) -> Chromosome<G::Allele> {
        let genes = self.random_genes_factory(rng);
        Chromosome::new(genes)
    }

    fn chromosome_cloner(&mut self, chromosome: &Chromosome<G::Allele>) -> Chromosome<G::Allele> {
        chromosome.clone()
    }

    fn chromosome_destructor_truncate(
        &mut self,
        chromosomes: &mut Vec<Chromosome<G::Allele>>,
        target_population_size: usize,
    ) {
        chromosomes.truncate(target_population_size);
    }

    fn chromosome_cloner_expand(
        &mut self,
        chromosomes: &mut Vec<Chromosome<G::Allele>>,
        amount: usize,
    ) {
        let modulo = chromosomes.len();
        for i in 0..amount {
            let chromosome = &chromosomes[i % modulo];
            chromosomes.push(chromosome.clone());
        }
    }
}
