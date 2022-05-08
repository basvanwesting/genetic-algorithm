use super::Genotype;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromGenotypeBuilderError(pub &'static str);

#[derive(Clone, Debug)]
pub struct Builder<G: Genotype> {
    pub gene_size: Option<usize>,
    pub gene_value_size: Option<<G as Genotype>::Gene>,
    pub gene_value_sizes: Vec<<G as Genotype>::Gene>,
    pub gene_values: Vec<<G as Genotype>::Gene>,
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

    pub fn with_gene_values(mut self, gene_values: Vec<<G as Genotype>::Gene>) -> Self {
        self.gene_values = gene_values;
        self
    }

    pub fn with_gene_value_sizes(mut self, gene_value_sizes: Vec<<G as Genotype>::Gene>) -> Self {
        self.gene_value_sizes = gene_value_sizes;
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
            gene_value_sizes: vec![],
            gene_values: vec![],
        }
    }
}
