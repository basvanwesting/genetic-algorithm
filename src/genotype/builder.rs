use super::Genotype;
use crate::chromosome::Genes;
pub use crate::errors::TryFromGenotypeBuilderError as TryFromBuilderError;
use std::ops::RangeInclusive;

/// The builder for a Genotype struct.
/// See specfic [Genotype] for used options.
///
/// Shared initialization options for all Genotypes:
/// * Builder `with_seed_genes_list(Vec<Genotype::Genes>)`, optional, list of start genes for
///   chromosomes which are cycled into the starting population until the target_population_size is
///   met (instead of the default random genes). Sometimes it is efficient to start with a certain
///   population in the Evolve strategy. For the HillClimb strategy a single random seed genes is
///   taken as the starting point for each run (not cycling through them in repeated runs).
///
/// * Builder `with_genes_hashing(true)`, optional, store a genes_hash on the chromomose (in Evolve
///   or HillClimb). This is needed when using `with_fitness_cache` on the strategy as key for the
///   cache. Hashing the genes has relatively high overhead for to the main Evolve loop, but might be
///   useful for better population cardinality estimation (falls back to fitness score cardinality
///   otherwise).
///
/// * Builder `with_chromosome_recycling(true)`, optional, recycle chromosome population instead of
///   reallocating repeatedly. Can be beneficiary for large genes_size. But does make the custom
///   implementations of Crossover require to handle this, otherwise a memory leak would occur
///
#[derive(Clone, Debug)]
pub struct Builder<G: Genotype> {
    pub genes_size: Option<usize>,
    pub allele_list: Option<Vec<G::Allele>>,
    pub allele_lists: Option<Vec<Vec<G::Allele>>>,
    pub allele_range: Option<RangeInclusive<G::Allele>>,
    pub allele_ranges: Option<Vec<RangeInclusive<G::Allele>>>,
    pub allele_mutation_range: Option<RangeInclusive<G::Allele>>,
    pub allele_mutation_ranges: Option<Vec<RangeInclusive<G::Allele>>>,
    pub allele_mutation_scaled_range: Option<Vec<RangeInclusive<G::Allele>>>,
    pub allele_mutation_scaled_ranges: Option<Vec<Vec<RangeInclusive<G::Allele>>>>,
    pub seed_genes_list: Vec<Genes<G::Allele>>,
    pub genes_hashing: bool,
    pub chromosome_recycling: bool,
}

impl<G: Genotype> Builder<G> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_genes_size(mut self, genes_size: usize) -> Self {
        self.genes_size = Some(genes_size);
        self
    }

    pub fn with_allele_list(mut self, allele_list: Vec<G::Allele>) -> Self {
        self.allele_list = Some(allele_list);
        self
    }

    pub fn with_allele_lists(mut self, allele_lists: Vec<Vec<G::Allele>>) -> Self {
        self.allele_lists = Some(allele_lists);
        self
    }

    pub fn with_allele_range(mut self, allele_range: RangeInclusive<G::Allele>) -> Self {
        self.allele_range = Some(allele_range);
        self
    }

    pub fn with_allele_ranges(mut self, allele_ranges: Vec<RangeInclusive<G::Allele>>) -> Self {
        self.allele_ranges = Some(allele_ranges);
        self
    }

    pub fn with_allele_mutation_range(
        mut self,
        allele_mutation_range: RangeInclusive<G::Allele>,
    ) -> Self {
        self.allele_mutation_range = Some(allele_mutation_range);
        self
    }

    pub fn with_allele_mutation_ranges(
        mut self,
        allele_mutation_ranges: Vec<RangeInclusive<G::Allele>>,
    ) -> Self {
        self.allele_mutation_ranges = Some(allele_mutation_ranges);
        self
    }

    pub fn with_allele_mutation_scaled_range(
        mut self,
        allele_mutation_scaled_range: Vec<RangeInclusive<G::Allele>>,
    ) -> Self {
        self.allele_mutation_scaled_range = Some(allele_mutation_scaled_range);
        self
    }

    pub fn with_allele_mutation_scaled_ranges(
        mut self,
        allele_mutation_scaled_ranges: Vec<Vec<RangeInclusive<G::Allele>>>,
    ) -> Self {
        self.allele_mutation_scaled_ranges = Some(allele_mutation_scaled_ranges);
        self
    }

    pub fn with_seed_genes_list(mut self, seed_genes_list: Vec<Genes<G::Allele>>) -> Self {
        self.seed_genes_list = seed_genes_list;
        self
    }

    pub fn with_genes_hashing(mut self, genes_hashing: bool) -> Self {
        self.genes_hashing = genes_hashing;
        self
    }

    pub fn with_chromosome_recycling(mut self, chromosome_recycling: bool) -> Self {
        self.chromosome_recycling = chromosome_recycling;
        self
    }

    pub fn build(self) -> Result<G, <G as TryFrom<Builder<G>>>::Error> {
        self.try_into()
    }
}

impl<G: Genotype> Default for Builder<G> {
    fn default() -> Self {
        Self {
            genes_size: None,
            allele_list: None,
            allele_lists: None,
            allele_range: None,
            allele_ranges: None,
            allele_mutation_range: None,
            allele_mutation_ranges: None,
            allele_mutation_scaled_range: None,
            allele_mutation_scaled_ranges: None,
            seed_genes_list: vec![],
            genes_hashing: false,
            chromosome_recycling: false,
        }
    }
}
