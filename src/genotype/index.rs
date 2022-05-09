use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub type IndexGene = usize;

#[derive(Clone, Debug)]
pub struct Index {
    pub gene_size: usize,
    pub gene_value_size: IndexGene,
    gene_index_sampler: Uniform<usize>,
    gene_value_sampler: Uniform<IndexGene>,
    pub seed_genes: Option<Vec<IndexGene>>,
}

impl TryFrom<Builder<Self>> for Index {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_size.is_none() {
            Err(TryFromBuilderError("IndexGenotype requires a gene_size"))
        } else if builder.gene_value_size.is_none() {
            Err(TryFromBuilderError(
                "IndexGenotype requires a gene_value_size",
            ))
        } else {
            let gene_size = builder.gene_size.unwrap();
            let gene_value_size = builder.gene_value_size.unwrap();

            Ok(Self {
                gene_size: gene_size,
                gene_value_size: gene_value_size,
                gene_index_sampler: Uniform::from(0..gene_size),
                gene_value_sampler: Uniform::from(0..gene_value_size),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for Index {
    type Gene = IndexGene;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Gene> = (0..self.gene_size)
                .map(|_| self.gene_value_sampler.sample(rng))
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.gene_value_sampler.sample(rng);
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype for Index {
    fn gene_values(&self) -> Vec<Self::Gene> {
        (0..self.gene_value_size).collect()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)?;
        writeln!(f, "  gene_value_size: {}", self.gene_value_size)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)?;
        writeln!(f, "  gene_value_sampler: {:?}", self.gene_value_sampler)
    }
}
