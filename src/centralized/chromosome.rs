//! The chromosome is a container for the genes and stores some useful values

mod row;

pub use self::row::Row as DynamicRangeChromosome;
pub use self::row::Row as StaticBinaryChromosome;
pub use self::row::Row as StaticRangeChromosome;

use crate::centralized::fitness::FitnessValue;
use crate::centralized::genotype::Genotype;
use rand::prelude::*;

/// The GenesHash is used for determining cardinality in the population
/// It could also be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesHash = u64;

/// The Chromosome is used as an individual in the [Population](crate::population::Population).
/// Chromosomes [select](crate::select), [crossover](crate::crossover) and [mutate](crate::mutate)
/// with each other in the [Evolve](crate::strategy::evolve::Evolve) strategy.
/// Each [Genotype] has its own associated [Chromosome] type.
///
/// In the centralized module, chromosomes don't own their genes - they just point to genes
/// stored in the Genotype's matrix. Use [Strategy::best_genes()](crate::strategy::Strategy::best_genes) or
/// [Strategy::best_genes_and_fitness_score()](crate::strategy::Strategy::best_genes_and_fitness_score)
/// to access gene data.
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
    fn reset_state(&mut self, genes_hash: Option<GenesHash>);
    fn copy_state(&mut self, other: &Self);
}
pub trait GenesPointer: Chromosome {
    fn new(row_id: usize) -> Self;
}

pub trait ChromosomeManager<G: Genotype> {
    /// Mandatory, random genes unless seed genes are provided
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> G::Genes;
    /// Mandatory, also copies state
    fn copy_genes(&mut self, source: &G::Chromosome, target: &mut G::Chromosome);
    /// Mandatory, also resets state
    fn set_genes(&mut self, chromosome: &mut G::Chromosome, genes: &G::Genes);
    /// Mandatory
    fn get_genes(&self, chromosome: &G::Chromosome) -> G::Genes;
    /// Mandatory
    fn chromosome_bin_push(&mut self, _chromosome: G::Chromosome);
    /// Mandatory
    /// Take from the recycling bin or create new chromosome with capacities set.
    /// Raise on empty bin here if fixed number of chromosomes is used
    fn chromosome_bin_find_or_create(&mut self) -> G::Chromosome;

    /// Provided, override if recycling bin needs setup
    fn chromosomes_setup(&mut self) {}
    /// Provided, override if recycling bin needs cleanup
    fn chromosomes_cleanup(&mut self) {}

    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut G::Chromosome, rng: &mut R) {
        let genes = self.random_genes_factory(rng);
        self.set_genes(chromosome, &genes);
    }
    fn chromosome_constructor_genes(&mut self, genes: &G::Genes) -> G::Chromosome {
        let mut chromosome = self.chromosome_bin_find_or_create();
        self.set_genes(&mut chromosome, genes);
        chromosome
    }
    fn chromosome_constructor_random<R: Rng>(&mut self, rng: &mut R) -> G::Chromosome {
        let genes = self.random_genes_factory(rng);
        self.chromosome_constructor_genes(&genes)
    }
    fn chromosome_cloner(&mut self, chromosome: &G::Chromosome) -> G::Chromosome {
        let mut new_chromosome = self.chromosome_bin_find_or_create();
        self.copy_genes(chromosome, &mut new_chromosome);
        new_chromosome
    }
    fn chromosome_destructor(&mut self, chromosome: G::Chromosome) {
        self.chromosome_bin_push(chromosome)
    }
    fn chromosome_destructor_truncate(
        &mut self,
        chromosomes: &mut Vec<G::Chromosome>,
        target_population_size: usize,
    ) {
        chromosomes
            .drain(target_population_size..)
            .for_each(|c| self.chromosome_destructor(c));
    }
    fn chromosome_cloner_expand(&mut self, chromosomes: &mut Vec<G::Chromosome>, amount: usize) {
        // maybe use cycle here, but this is oddly elegant as the modulo ensures the newly pushed
        // chromosomes are never in the cycled selection
        let modulo = chromosomes.len();
        for i in 0..amount {
            let chromosome = &chromosomes[i % modulo];
            chromosomes.push(self.chromosome_cloner(chromosome));
        }
    }
}
