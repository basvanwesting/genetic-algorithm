use super::Genotype;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

#[derive(Clone, Debug)]
pub struct Builder<G: Genotype> {
    pub gene_size: Option<usize>,
    pub gene_value_size: Option<<G as Genotype>::Gene>,
    pub gene_value_sizes: Option<Vec<<G as Genotype>::Gene>>,
    pub gene_values: Option<Vec<<G as Genotype>::Gene>>,
    pub gene_value_offset: Option<<G as Genotype>::Gene>,
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

    pub fn with_gene_value_size(mut self, gene_value_size: <G as Genotype>::Gene) -> Self {
        self.gene_value_size = Some(gene_value_size);
        self
    }

    pub fn with_gene_value_offset(mut self, gene_value_offset: <G as Genotype>::Gene) -> Self {
        self.gene_value_offset = Some(gene_value_offset);
        self
    }

    pub fn with_gene_values(mut self, gene_values: Vec<<G as Genotype>::Gene>) -> Self {
        self.gene_values = Some(gene_values);
        self
    }

    pub fn with_seed_genes(mut self, seed_genes: Vec<<G as Genotype>::Gene>) -> Self {
        self.seed_genes = Some(seed_genes);
        self
    }

    pub fn with_gene_value_sizes(mut self, gene_value_sizes: Vec<<G as Genotype>::Gene>) -> Self {
        self.gene_value_sizes = Some(gene_value_sizes);
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
            gene_value_offset: None,
            seed_genes: None,
        }
    }
}
