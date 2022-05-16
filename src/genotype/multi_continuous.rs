use super::builder::{Builder, TryFromBuilderError};
use super::Genotype;
use crate::chromosome::Chromosome;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;
use std::ops::Range;

pub type ContinuousAllele = f32;

/// Genes are a list of f32, each individually taken from its own allele_range. The gene_size is
/// derived to be the allele_multi_range length. On random initialization, each gene gets a value
/// from its own allele_range with a uniform probability. Each gene has a weighted probability of
/// mutating, depending on its allele_range size. If a gene mutates, a new values is taken from its
/// own allele_range with a uniform probability. Duplicate allele values are allowed. Defaults to usize
/// as item.
///
/// # Example:
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiContinuousGenotype};
///
/// let genotype = MultiContinuousGenotype::builder()
///     .with_allele_multi_range(vec![
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
    pub allele_multi_range: Vec<Range<ContinuousAllele>>,
    gene_index_sampler: WeightedIndex<ContinuousAllele>,
    allele_value_samplers: Vec<Uniform<ContinuousAllele>>,
    pub seed_genes: Option<Vec<ContinuousAllele>>,
}

impl TryFrom<Builder<Self>> for MultiContinuous {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_multi_range.is_none() {
            Err(TryFromBuilderError(
                "MultiContinuousGenotype requires a allele_multi_range",
            ))
        } else if builder
            .allele_multi_range
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "MultiContinuousGenotype requires non-empty allele_multi_range",
            ))
        } else {
            let allele_multi_range = builder.allele_multi_range.unwrap();
            let gene_size = allele_multi_range.len();
            let index_weights: Vec<ContinuousAllele> = allele_multi_range
                .iter()
                .map(|allele_range| allele_range.end - allele_range.start)
                .collect();

            Ok(Self {
                gene_size: gene_size,
                allele_multi_range: allele_multi_range.clone(),
                gene_index_sampler: WeightedIndex::new(index_weights).unwrap(),
                allele_value_samplers: allele_multi_range
                    .iter()
                    .map(|allele_range| Uniform::from(allele_range.clone()))
                    .collect(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for MultiContinuous {
    type Allele = ContinuousAllele;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Allele> = (0..self.gene_size)
                .map(|index| self.allele_value_samplers[index].sample(rng))
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.allele_value_samplers[index].sample(rng);
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for MultiContinuous {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}", self.gene_size)?;
        writeln!(f, "  allele_multi_range: {:?}\n", self.allele_multi_range)?;
        writeln!(f, "  chromosome_permutations_size: uncountable")?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
