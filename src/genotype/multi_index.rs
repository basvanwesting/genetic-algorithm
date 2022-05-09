use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::population::Population;
use itertools::Itertools;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

pub type IndexGene = usize;

#[derive(Clone, Debug)]
pub struct MultiIndex {
    gene_size: usize,
    pub gene_value_sizes: Vec<IndexGene>,
    gene_index_sampler: WeightedIndex<usize>,
    gene_value_samplers: Vec<Uniform<IndexGene>>,
    pub seed_genes: Option<Vec<IndexGene>>,
}

impl TryFrom<Builder<Self>> for MultiIndex {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_value_sizes.is_none() {
            Err(TryFromBuilderError(
                "MultiIndexGenotype requires a gene_value_sizes",
            ))
        } else if builder
            .gene_value_sizes
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "UniqueDiscreteGenotype requires non-empty gene_value_sizes",
            ))
        } else {
            let gene_value_sizes = builder.gene_value_sizes.unwrap();
            Ok(Self {
                gene_size: gene_value_sizes.len(),
                gene_value_sizes: gene_value_sizes.clone(),
                gene_index_sampler: WeightedIndex::new(gene_value_sizes.clone()).unwrap(),
                gene_value_samplers: gene_value_sizes
                    .iter()
                    .map(|gene_value_size| Uniform::from(0..*gene_value_size))
                    .collect(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for MultiIndex {
    type Gene = IndexGene;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Gene> = (0..self.gene_size)
                .map(|index| self.gene_value_samplers[index].sample(rng))
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.gene_value_samplers[index].sample(rng);
        chromosome.taint_fitness_score();
    }
}

impl PermutableGenotype for MultiIndex {
    //noop
    fn gene_values(&self) -> Vec<Self::Gene> {
        vec![]
    }
    fn population_factory(&self) -> Population<Self> {
        let chromosomes = self
            .gene_value_sizes
            .iter()
            .map(|gene_value_size| (0..*gene_value_size).collect::<Vec<Self::Gene>>())
            .multi_cartesian_product()
            .map(Chromosome::new)
            .collect();

        Population::new(chromosomes)
    }
    fn population_factory_size(&self) -> usize {
        self.gene_value_sizes.iter().product()
    }
}

impl fmt::Display for MultiIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}\n", self.gene_size)?;
        writeln!(f, "  gene_value_sizes: {:?}", self.gene_value_sizes)?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
