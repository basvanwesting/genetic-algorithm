use super::Genotype;
pub use crate::errors::TryFromGenotypeBuilderError as TryFromBuilderError;
use std::ops::RangeInclusive;

/// The builder for a Genotype struct.
/// See specfic [Genotype] for used options.
///
/// Shared initialization options for all Genotypes:
/// * Builder `with_seed_genes_list(Vec<Genotype::Genes>)`, optional, list of start genes of all chromosomes
///   which are distributed randomly in the population (instead of the default random genes).
///   Sometimes it is efficient to start with a certain population
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
    pub mutation_gene_index_weights: Option<Vec<f64>>,
    pub crossover_gene_index_weights: Option<Vec<f64>>,
    pub crossover_point_index_weights: Option<Vec<f64>>,
    pub seed_genes_list: Vec<G::Genes>,
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

    pub fn with_seed_genes_list(mut self, seed_genes_list: Vec<G::Genes>) -> Self {
        self.seed_genes_list = seed_genes_list;
        self
    }

    pub fn with_mutation_gene_index_weights<T: Into<f64>>(mut self, weights: Vec<T>) -> Self {
        self.mutation_gene_index_weights = Some(weights.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_crossover_gene_index_weights<T: Into<f64>>(mut self, weights: Vec<T>) -> Self {
        self.crossover_gene_index_weights = Some(weights.into_iter().map(Into::into).collect());
        self
    }

    pub fn with_crossover_point_index_weights<T: Into<f64>>(mut self, weights: Vec<T>) -> Self {
        self.crossover_point_index_weights = Some(weights.into_iter().map(Into::into).collect());
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
            mutation_gene_index_weights: None,
            crossover_gene_index_weights: None,
            crossover_point_index_weights: None,
            seed_genes_list: vec![],
        }
    }
}
