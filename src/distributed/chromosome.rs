//! The chromosome is a container for the genes and stores some useful values

mod vector;

pub use self::vector::Vector as VecChromosome;

use crate::distributed::fitness::FitnessValue;
use crate::distributed::genotype::Genotype;
use rand::prelude::*;

/// The GenesHash is used for determining cardinality in the population
/// It could also be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesHash = u64;

/// The Chromosome is used as an individual in the [Population](crate::population::Population).
/// Chromosomes [select](crate::select), [crossover](crate::crossover) and [mutate](crate::mutate)
/// with each other in the [Evolve](crate::strategy::evolve::Evolve) strategy.
/// Each [Genotype] has its own associated [Chromosome] type.
///
/// In the distributed module, chromosomes implement [GenesOwner] and own their genes directly.
/// You can use [Evolve::best_chromosome()](crate::strategy::evolve::Evolve::best_chromosome),
/// [HillClimb::best_chromosome()](crate::strategy::hill_climb::HillClimb::best_chromosome) and
/// [Permutate::best_chromosome()](crate::strategy::permutate::Permutate::best_chromosome)
/// to access the best chromosome directly.
pub trait Chromosome: Clone + Send {
    fn age(&self) -> usize;
    fn reset_age(&mut self);
    fn increment_age(&mut self);
    fn set_age(&mut self, age: usize);
    fn is_offspring(&self) -> bool;
    fn fitness_score(&self) -> Option<FitnessValue>;
    fn set_fitness_score(&mut self, fitness_score: Option<FitnessValue>);
    fn genes_hash(&self) -> Option<GenesHash>;
    fn set_genes_hash(&mut self, genes_hash: Option<GenesHash>);
    fn update_state(&mut self);
    fn reset_state(&mut self, genes_hash: Option<GenesHash>);
    fn copy_state(&mut self, other: &Self);
}
pub trait GenesOwner: Chromosome {
    type Genes: Clone + Send + Sync + std::fmt::Debug;
    fn new(genes: Self::Genes) -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn genes(&self) -> &Self::Genes;
    fn get_genes(&self) -> Self::Genes;
    fn set_genes(&mut self, genes: Self::Genes);
    fn copy_from(&mut self, source: &Self);
}

pub trait ChromosomeManager<G: Genotype> {
    /// Create random genes based on genotype configuration
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> G::Genes;
    /// Get the capacity hint for creating new chromosomes
    fn genes_capacity(&self) -> usize;

    // Helper methods using the new chromosome capabilities
    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut G::Chromosome, rng: &mut R)
    where
        G::Chromosome: GenesOwner<Genes = G::Genes>,
    {
        let genes = self.random_genes_factory(rng);
        chromosome.set_genes(genes);
    }

    fn chromosome_constructor_genes(&mut self, genes: &G::Genes) -> G::Chromosome
    where
        G::Chromosome: GenesOwner<Genes = G::Genes>,
    {
        G::Chromosome::new(genes.clone())
    }

    fn chromosome_constructor_random<R: Rng>(&mut self, rng: &mut R) -> G::Chromosome
    where
        G::Chromosome: GenesOwner<Genes = G::Genes>,
    {
        let genes = self.random_genes_factory(rng);
        G::Chromosome::new(genes)
    }

    fn chromosome_cloner(&mut self, chromosome: &G::Chromosome) -> G::Chromosome
    where
        G::Chromosome: Clone,
    {
        chromosome.clone()
    }

    fn chromosome_destructor_truncate(
        &mut self,
        chromosomes: &mut Vec<G::Chromosome>,
        target_population_size: usize,
    ) {
        chromosomes.truncate(target_population_size);
    }

    fn chromosome_cloner_expand(&mut self, chromosomes: &mut Vec<G::Chromosome>, amount: usize)
    where
        G::Chromosome: Clone,
    {
        let modulo = chromosomes.len();
        for i in 0..amount {
            let chromosome = &chromosomes[i % modulo];
            chromosomes.push(chromosome.clone());
        }
    }
}
