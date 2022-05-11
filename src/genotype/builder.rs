use super::Genotype;
use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for a Genotype struct
///
/// Shared initialization options for all Genotypes:
/// * Builder `with_seed_genes(Vec<_>)`, optional, start genes of all chromosomes in the population
///   (instead of the default random genes). Sometimes it is efficient to start with a certain population
///   (e.g. [Knapsack problem](../main/examples/evolve_knapsack.rs) with no items in it)
///
#[derive(Clone, Debug)]
pub struct Builder<G: Genotype> {
    pub gene_size: Option<usize>,
    pub gene_value_size: Option<<G as Genotype>::Gene>,
    pub gene_value_sizes: Option<Vec<<G as Genotype>::Gene>>,
    pub gene_values: Option<Vec<<G as Genotype>::Gene>>,
    pub gene_multi_values: Option<Vec<Vec<<G as Genotype>::Gene>>>,
    pub gene_range: Option<Range<<G as Genotype>::Gene>>,
    pub gene_ranges: Option<Vec<Range<<G as Genotype>::Gene>>>,
    pub seed_genes: Option<Vec<<G as Genotype>::Gene>>,
}

impl<G: Genotype> Builder<G> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = Some(gene_size);
        self
    }

    pub fn with_gene_values(mut self, gene_values: Vec<<G as Genotype>::Gene>) -> Self {
        self.gene_values = Some(gene_values);
        self
    }

    pub fn with_gene_multi_values(
        mut self,
        gene_multi_values: Vec<Vec<<G as Genotype>::Gene>>,
    ) -> Self {
        self.gene_multi_values = Some(gene_multi_values);
        self
    }

    pub fn with_gene_range(mut self, gene_range: Range<<G as Genotype>::Gene>) -> Self {
        self.gene_range = Some(gene_range);
        self
    }

    pub fn with_gene_ranges(mut self, gene_ranges: Vec<Range<<G as Genotype>::Gene>>) -> Self {
        self.gene_ranges = Some(gene_ranges);
        self
    }

    pub fn with_seed_genes(mut self, seed_genes: Vec<<G as Genotype>::Gene>) -> Self {
        self.seed_genes = Some(seed_genes);
        self
    }

    pub fn build(self) -> Result<G, <G as TryFrom<Builder<G>>>::Error> {
        self.try_into()
    }
}

impl<G: Genotype> Default for Builder<G> {
    fn default() -> Self {
        Self {
            gene_size: None,
            gene_value_size: None,
            gene_value_sizes: None,
            gene_values: None,
            gene_multi_values: None,
            gene_range: None,
            gene_ranges: None,
            seed_genes: None,
        }
    }
}
