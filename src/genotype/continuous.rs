use super::builder::{Builder, TryFromBuilderError};
use super::Genotype;
use crate::chromosome::Chromosome;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;
use std::ops::Range;

// trait alias, experimental
//pub trait Gene = Clone + std::fmt::Debug;

pub type DefaultContinuousGene = f32;
pub struct Continuous<T: Clone + std::fmt::Debug + SampleUniform = DefaultContinuousGene> {
    pub gene_size: usize,
    pub gene_range: Range<T>,
    gene_index_sampler: Uniform<usize>,
    gene_value_sampler: Uniform<T>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug + SampleUniform> TryFrom<Builder<Self>> for Continuous<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_size.is_none() {
            Err(TryFromBuilderError(
                "ContinuousGenotype requires a gene_size",
            ))
        } else if builder.gene_range.is_none() {
            Err(TryFromBuilderError(
                "ContinuousGenotype requires a gene_range",
            ))
        } else {
            let gene_size = builder.gene_size.unwrap();
            let gene_range = builder.gene_range.unwrap();

            Ok(Self {
                gene_size: gene_size,
                gene_range: gene_range.clone(),
                gene_index_sampler: Uniform::from(0..gene_size),
                gene_value_sampler: Uniform::from(gene_range.clone()),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Clone + std::fmt::Debug + SampleUniform> Genotype for Continuous<T> {
    type Gene = T;
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

impl<T: Clone + std::fmt::Debug + SampleUniform> Clone for Continuous<T> {
    fn clone(&self) -> Self {
        Self {
            gene_size: self.gene_size,
            gene_range: self.gene_range.clone(),
            gene_index_sampler: Uniform::from(0..self.gene_size),
            gene_value_sampler: Uniform::from(self.gene_range.clone()),
            seed_genes: self.seed_genes.clone(),
        }
    }
}

impl<T: Clone + std::fmt::Debug + SampleUniform> fmt::Display for Continuous<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)
    }
}

impl<T: Clone + std::fmt::Debug + SampleUniform> fmt::Debug for Continuous<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)
    }
}
