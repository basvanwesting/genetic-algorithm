use super::Genotype;
use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for a Genotype struct
///
/// Shared initialization options for all Genotypes:
/// * Builder `with_seed_genes(Vec<_>)`, optional, start genes of all chromosomes in the population
///   (instead of the default random genes). Sometimes it is efficient to start with a certain population
///
#[derive(Clone, Debug)]
pub struct Builder<G: Genotype> {
    pub genes_size: Option<usize>,
    pub allele_values: Option<Vec<<G as Genotype>::Allele>>,
    pub allele_multi_values: Option<Vec<Vec<<G as Genotype>::Allele>>>,
    pub allele_range: Option<Range<<G as Genotype>::Allele>>,
    pub allele_multi_range: Option<Vec<Range<<G as Genotype>::Allele>>>,
    pub allele_neighbour_range: Option<Range<<G as Genotype>::Allele>>,
    pub seed_genes: Option<Vec<<G as Genotype>::Allele>>,
}

impl<G: Genotype> Builder<G> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_genes_size(mut self, genes_size: usize) -> Self {
        self.genes_size = Some(genes_size);
        self
    }

    pub fn with_allele_values(mut self, allele_values: Vec<<G as Genotype>::Allele>) -> Self {
        self.allele_values = Some(allele_values);
        self
    }

    pub fn with_allele_multi_values(
        mut self,
        allele_multi_values: Vec<Vec<<G as Genotype>::Allele>>,
    ) -> Self {
        self.allele_multi_values = Some(allele_multi_values);
        self
    }

    pub fn with_allele_range(mut self, allele_range: Range<<G as Genotype>::Allele>) -> Self {
        self.allele_range = Some(allele_range);
        self
    }

    pub fn with_allele_multi_range(
        mut self,
        allele_multi_range: Vec<Range<<G as Genotype>::Allele>>,
    ) -> Self {
        self.allele_multi_range = Some(allele_multi_range);
        self
    }

    pub fn with_allele_neighbour_range(
        mut self,
        allele_neighbour_range: Range<<G as Genotype>::Allele>,
    ) -> Self {
        self.allele_neighbour_range = Some(allele_neighbour_range);
        self
    }

    pub fn with_seed_genes(mut self, seed_genes: Vec<<G as Genotype>::Allele>) -> Self {
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
            genes_size: None,
            allele_values: None,
            allele_multi_values: None,
            allele_range: None,
            allele_multi_range: None,
            allele_neighbour_range: None,
            seed_genes: None,
        }
    }
}
