//! The chromosome is a container for the genes and stores some useful values

mod chromosome_struct;

pub use self::chromosome_struct::Chromosome;

// Legacy type aliases for backwards compatibility (will be removed later)
pub use self::chromosome_struct::Chromosome as DynamicRangeChromosome;
pub use self::chromosome_struct::Chromosome as StaticBinaryChromosome;
pub use self::chromosome_struct::Chromosome as StaticRangeChromosome;

use crate::centralized::genotype::Genotype;
use rand::prelude::*;

/// The GenesHash is used for determining cardinality in the population
/// It could also be used for caching fitness scores, without lifetime concerns of the chromosome
pub type GenesHash = u64;

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
