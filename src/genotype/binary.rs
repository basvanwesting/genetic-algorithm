use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;
use std::fmt;

pub type BinaryGene = bool;

#[derive(Clone, Debug)]
pub struct Binary {
    pub gene_size: usize,
    gene_index_sampler: Uniform<usize>,
    gene_value_sampler: Bernoulli,
    pub seed_genes: Option<Vec<BinaryGene>>,
}

impl TryFrom<Builder<Self>> for Binary {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_size.is_none() {
            Err(TryFromBuilderError("BinaryGenotype requires a gene_size"))
        } else {
            Ok(Self {
                gene_size: builder.gene_size.unwrap(),
                gene_index_sampler: Uniform::from(0..builder.gene_size.unwrap()),
                gene_value_sampler: Bernoulli::new(0.5).unwrap(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for Binary {
    type Gene = BinaryGene;
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
        chromosome.genes[index] = !chromosome.genes[index];
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype for Binary {
    fn gene_values(&self) -> Vec<Self::Gene> {
        vec![true, false]
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)?;
        writeln!(f, "  gene_index_sampler: {:?}", self.gene_index_sampler)?;
        writeln!(f, "  gene_value_sampler: {:?}", self.gene_value_sampler)
    }
}
