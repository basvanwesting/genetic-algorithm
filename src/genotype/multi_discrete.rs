use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

pub type DefaultDiscreteGene = usize;

/// Genes are a list of values, each individually taken from its own gene_values using clone(). The
/// gene_size is derived to be the gene_multi_values length. All gene_values have to be of the same
/// type, but can have different values and lengths. On random initialization, each gene gets a
/// value from its own gene_values with a uniform probability. Each gene has a weighted probability
/// of mutating, depending on its gene_values length. If a gene mutates, a new values is taken from
/// its own gene_values with a uniform probability (regardless of current value, which could
/// therefore be assigned again, not mutating as a result). Duplicate gene values are allowed.
/// Defaults to usize as item.
///
/// This genotype is also used in the [meta analysis](crate::meta), to hold the indices of the
/// different [Evolve](crate::evolve::Evolve) configuration values (defined outside of the genotype).
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiDiscreteGenotype};
///
/// let genotype = MultiDiscreteGenotype::builder()
///     .with_gene_multi_values(vec![
///        (0..10).collect(),
///        (0..20).collect(),
///        (0..5).collect(),
///        (0..30).collect(),
///     ])
///     .build()
///     .unwrap();
/// ```
///
/// # Example (usize, used to lookup external types of different kind):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiDiscreteGenotype};
///
/// let cars = vec!["BMW X3", "Ford Mustang", "Chevrolet Camaro"];
/// let drivers = vec!["Louis", "Max", "Charles"];
/// let number_of_laps = vec![10, 20, 30, 40];
/// let rain_probabilities = vec![0.0, 0.2, 0.4, 0.6, 0.8, 1.0];
///
/// let genotype = MultiDiscreteGenotype::builder()
///     .with_gene_multi_values(vec![
///        (0..cars.len()).collect(),
///        (0..drivers.len()).collect(),
///        (0..number_of_laps.len()).collect(),
///        (0..rain_probabilities.len()).collect(),
///     ])
///     .build()
///     .unwrap();
///
/// // The fitness function will be provided the genes (e.g. [2,0,1,4]) and will then have to
/// // lookup the external types and implement some fitness logic for the combination (e.g.
/// // ["Chevrolet Camaro", "Louis", 20, 0.8])
/// ```
///
/// # Example (struct, the limitation is that the type needs to be the same for all lists)
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiDiscreteGenotype};
///
/// #[derive(Clone, Debug)]
/// struct Item(pub u16, pub u16);
///
/// let genotype = MultiDiscreteGenotype::builder()
///     .with_gene_multi_values(vec![
///       vec![Item(23, 505), Item(26, 352), Item(20, 458)],
///       vec![Item(23, 505), Item(26, 352)],
///       vec![Item(26, 352), Item(20, 458), Item(13, 123)],
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct MultiDiscrete<T: Clone + std::fmt::Debug = DefaultDiscreteGene> {
    gene_size: usize,
    gene_value_sizes: Vec<usize>,
    pub gene_multi_values: Vec<Vec<T>>,
    gene_index_sampler: WeightedIndex<usize>,
    gene_value_index_samplers: Vec<Uniform<usize>>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug> TryFrom<Builder<Self>> for MultiDiscrete<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_multi_values.is_none() {
            Err(TryFromBuilderError(
                "MultiDiscreteGenotype requires a gene_multi_values",
            ))
        } else if builder
            .gene_multi_values
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "MultiDiscreteGenotype requires non-empty gene_multi_values",
            ))
        } else {
            let gene_multi_values = builder.gene_multi_values.unwrap();
            let gene_value_sizes: Vec<usize> = gene_multi_values.iter().map(|v| v.len()).collect();
            Ok(Self {
                gene_size: gene_multi_values.len(),
                gene_value_sizes: gene_value_sizes.clone(),
                gene_multi_values: gene_multi_values.clone(),
                gene_index_sampler: WeightedIndex::new(gene_value_sizes.clone()).unwrap(),
                gene_value_index_samplers: gene_value_sizes
                    .iter()
                    .map(|gene_value_size| Uniform::from(0..*gene_value_size))
                    .collect(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Clone + std::fmt::Debug> Genotype for MultiDiscrete<T> {
    type Gene = T;
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Gene> = self
                .gene_multi_values
                .iter()
                .enumerate()
                .map(|(index, gene_values)| {
                    gene_values[self.gene_value_index_samplers[index].sample(rng)].clone()
                })
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.gene_multi_values[index]
            [self.gene_value_index_samplers[index].sample(rng)]
        .clone();
        chromosome.taint_fitness_score();
    }
}

impl<T: Clone + std::fmt::Debug> PermutableGenotype for MultiDiscrete<T> {
    //noop
    fn gene_values(&self) -> Vec<Self::Gene> {
        vec![]
    }

    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self>> + 'a> {
        Box::new(
            self.gene_multi_values
                .clone()
                .into_iter()
                .multi_cartesian_product()
                .map(Chromosome::new),
        )
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        self.gene_value_sizes
            .iter()
            .map(|v| BigUint::from(*v))
            .product()
    }
}

impl<T: Clone + std::fmt::Debug> fmt::Display for MultiDiscrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_size: {}\n", self.gene_size)?;
        writeln!(f, "  gene_value_sizes: {:?}", self.gene_value_sizes)?;
        writeln!(f, "  gene_multi_values: {:?}", self.gene_multi_values)?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
