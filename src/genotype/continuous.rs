use super::builder::{Builder, TryFromBuilderError};
use super::Genotype;
use crate::chromosome::Chromosome;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub type ContinuousGene = f32;

#[derive(Clone, Debug)]
pub struct Continuous {
    pub gene_size: usize,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes: Option<Vec<ContinuousGene>>,
}

impl TryFrom<Builder<Self>> for Continuous {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_size.is_none() {
            Err(TryFromBuilderError(
                "ContinuousGenotype requires a gene_size",
            ))
        } else {
            Ok(Self {
                gene_size: builder.gene_size.unwrap(),
                gene_index_sampler: Uniform::from(0..builder.gene_size.unwrap()),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for Continuous {
    type Gene = ContinuousGene;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Gene> = (0..self.gene_size).map(|_| rng.gen()).collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = rng.gen();
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for Continuous {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)
    }
}
