use super::builder::{Builder, TryFromBuilderError};
use super::Genotype;
use crate::chromosome::Chromosome;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;
use std::ops::Range;

pub type ContinuousGene = f32;

/// Genes are a list of f32, each individually taken from its own gene_range. The gene_size is
/// derived to be the gene_multi_range length. On random initialization, each gene gets a value
/// from its own gene_range with a uniform probability. Each gene has a weighted probability of
/// mutating, depending on its gene_range size. If a gene mutates, a new values is taken from its
/// own gene_range with a uniform probability. Duplicate gene values are allowed. Defaults to usize
/// as item.
///
/// # Example:
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiContinuousGenotype};
///
/// let genotype = MultiContinuousGenotype::builder()
///     .with_gene_multi_range(vec![
///        (0.0..10.0),
///        (5.0..20.0),
///        (0.0..5.0),
///        (10.0..30.0),
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct MultiContinuous {
    gene_size: usize,
    pub gene_multi_range: Vec<Range<ContinuousGene>>,
    gene_index_sampler: WeightedIndex<ContinuousGene>,
    gene_value_samplers: Vec<Uniform<ContinuousGene>>,
    pub seed_genes: Option<Vec<ContinuousGene>>,
}

impl TryFrom<Builder<Self>> for MultiContinuous {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_multi_range.is_none() {
            Err(TryFromBuilderError(
                "MultiContinuousGenotype requires a gene_multi_range",
            ))
        } else if builder
            .gene_multi_range
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "MultiContinuousGenotype requires non-empty gene_multi_range",
            ))
        } else {
            let gene_multi_range = builder.gene_multi_range.unwrap();
            let gene_size = gene_multi_range.len();
            let index_weights: Vec<ContinuousGene> = gene_multi_range
                .iter()
                .map(|gene_range| gene_range.end - gene_range.start)
                .collect();

            Ok(Self {
                gene_size: gene_size,
                gene_multi_range: gene_multi_range.clone(),
                gene_index_sampler: WeightedIndex::new(index_weights).unwrap(),
                gene_value_samplers: gene_multi_range
                    .iter()
                    .map(|gene_range| Uniform::from(gene_range.clone()))
                    .collect(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for MultiContinuous {
    type Gene = ContinuousGene;
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

impl fmt::Display for MultiContinuous {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)?;
        writeln!(f, "  gene_multi_range: {:?}\n", self.gene_multi_range)?;
        writeln!(f, "  chromosome_permutations_size: uncountable")?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
