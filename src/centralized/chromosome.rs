//! The chromosome is a pointer to the genes and stores some useful values
use crate::centralized::fitness::FitnessValue;
use crate::centralized::genotype::Genotype;
use rand::prelude::*;

/// The GenesHash is used for determining cardinality in the population
/// It could also be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesHash = u64;

#[derive(Copy, Clone, Debug)]
pub struct Chromosome {
    pub row_id: usize,
    pub fitness_score: Option<FitnessValue>,
    pub genes_hash: Option<GenesHash>,
    pub age: usize,
}

impl Chromosome {
    pub fn new(row_id: usize) -> Self {
        Self {
            row_id,
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
}

pub trait ChromosomeManager<G: Genotype> {
    /// Mandatory, random genes unless seed genes are provided
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> G::Genes;
    /// Mandatory, also copies state
    fn copy_genes(&mut self, source: &Chromosome, target: &mut Chromosome);
    /// Mandatory, also resets state
    fn set_genes(&mut self, chromosome: &mut Chromosome, genes: &G::Genes);
    /// Mandatory
    fn get_genes(&self, chromosome: &Chromosome) -> G::Genes;
    /// Mandatory
    fn chromosome_bin_push(&mut self, _chromosome: Chromosome);
    /// Mandatory
    /// Take from the recycling bin or create new chromosome with capacities set.
    /// Raise on empty bin here if fixed number of chromosomes is used
    fn chromosome_bin_find_or_create(&mut self) -> Chromosome;

    /// Provided, override if recycling bin needs setup
    fn chromosomes_setup(&mut self) {}
    /// Provided, override if recycling bin needs cleanup
    fn chromosomes_cleanup(&mut self) {}

    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut Chromosome, rng: &mut R) {
        let genes = self.random_genes_factory(rng);
        self.set_genes(chromosome, &genes);
    }
    fn chromosome_constructor_genes(&mut self, genes: &G::Genes) -> Chromosome {
        let mut chromosome = self.chromosome_bin_find_or_create();
        self.set_genes(&mut chromosome, genes);
        chromosome
    }
    fn chromosome_constructor_random<R: Rng>(&mut self, rng: &mut R) -> Chromosome {
        let genes = self.random_genes_factory(rng);
        self.chromosome_constructor_genes(&genes)
    }
    fn chromosome_cloner(&mut self, chromosome: &Chromosome) -> Chromosome {
        let mut new_chromosome = self.chromosome_bin_find_or_create();
        self.copy_genes(chromosome, &mut new_chromosome);
        new_chromosome
    }
    fn chromosome_destructor(&mut self, chromosome: Chromosome) {
        self.chromosome_bin_push(chromosome)
    }
    fn chromosome_destructor_truncate(
        &mut self,
        chromosomes: &mut Vec<Chromosome>,
        target_population_size: usize,
    ) {
        chromosomes
            .drain(target_population_size..)
            .for_each(|c| self.chromosome_destructor(c));
    }
    fn chromosome_cloner_expand(&mut self, chromosomes: &mut Vec<Chromosome>, amount: usize) {
        // maybe use cycle here, but this is oddly elegant as the modulo ensures the newly pushed
        // chromosomes are never in the cycled selection
        let modulo = chromosomes.len();
        for i in 0..amount {
            let chromosome = &chromosomes[i % modulo];
            chromosomes.push(self.chromosome_cloner(chromosome));
        }
    }
}
