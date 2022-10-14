use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

pub type DefaultAllele = usize;

/// Genes are a list of values, each individually taken from its own allele_list using clone(). The
/// genes_size is derived to be the allele_lists length. All allele_list have to be of the same
/// type, but can have different values and lengths. On random initialization, each gene gets a
/// value from its own allele_list with a uniform probability. Each gene has a weighted probability
/// of mutating, depending on its allele_list length. If a gene mutates, a new values is taken from
/// its own allele_list with a uniform probability (regardless of current value, which could
/// therefore be assigned again, not mutating as a result). Duplicate allele values are allowed.
/// Defaults to usize as item.
///
/// This genotype is also used in the [meta analysis](crate::meta), to hold the indices of the
/// different [Evolve](crate::strategy::evolve::Evolve) configuration values (defined outside of the genotype).
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiDiscreteGenotype};
///
/// let genotype = MultiDiscreteGenotype::builder()
///     .with_allele_lists(vec![
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
///     .with_allele_lists(vec![
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
/// #[derive(PartialEq, Clone, Debug)]
/// struct Item(pub u16, pub u16);
///
/// let genotype = MultiDiscreteGenotype::builder()
///     .with_allele_lists(vec![
///       vec![Item(23, 505), Item(26, 352), Item(20, 458)],
///       vec![Item(23, 505), Item(26, 352)],
///       vec![Item(26, 352), Item(20, 458), Item(13, 123)],
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct MultiDiscrete<T: PartialEq + Clone + Send + std::fmt::Debug = DefaultAllele> {
    genes_size: usize,
    pub allele_lists: Vec<Vec<T>>,
    gene_index_sampler: WeightedIndex<usize>,
    allele_index_samplers: Vec<Uniform<usize>>,
    allele_list_sizes: Vec<usize>,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> TryFrom<Builder<Self>> for MultiDiscrete<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_lists.is_none() {
            Err(TryFromBuilderError(
                "MultiDiscreteGenotype requires a allele_lists",
            ))
        } else if builder.allele_lists.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "MultiDiscreteGenotype requires non-empty allele_lists",
            ))
        } else {
            let allele_lists = builder.allele_lists.unwrap();
            let allele_list_sizes: Vec<usize> = allele_lists.iter().map(|v| v.len()).collect();
            Ok(Self {
                genes_size: allele_lists.len(),
                allele_list_sizes: allele_list_sizes.clone(),
                allele_lists: allele_lists.clone(),
                gene_index_sampler: WeightedIndex::new(allele_list_sizes.clone()).unwrap(),
                allele_index_samplers: allele_list_sizes
                    .iter()
                    .map(|allele_value_size| Uniform::from(0..*allele_value_size))
                    .collect(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> Genotype for MultiDiscrete<T> {
    type Allele = T;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn chromosome_seed<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            self.chromosome_factory(rng)
        }
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            let mut chromosome = Chromosome::new(seed_genes.clone());
            self.mutate_chromosome_random(&mut chromosome, rng);
            chromosome
        } else {
            let genes: Vec<Self::Allele> = self
                .allele_lists
                .iter()
                .enumerate()
                .map(|(index, allele_list)| {
                    allele_list[self.allele_index_samplers[index].sample(rng)].clone()
                })
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome_random<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] =
            self.allele_lists[index][self.allele_index_samplers[index].sample(rng)].clone();
        chromosome.taint_fitness_score();
    }
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> IncrementalGenotype for MultiDiscrete<T> {
    fn mutate_chromosome_neighbour<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self>,
        _scale: Option<f32>,
        rng: &mut R,
    ) {
        self.mutate_chromosome_random(chromosome, rng);
    }

    fn neighbouring_population(
        &self,
        chromosome: &Chromosome<Self>,
        _scale: Option<f32>,
    ) -> Population<Self> {
        (0..self.genes_size)
            .flat_map(|index| {
                self.allele_lists[index]
                    .iter()
                    .filter_map(move |allele_value| {
                        if chromosome.genes[index] == *allele_value {
                            None
                        } else {
                            let mut genes = chromosome.genes.clone();
                            genes[index] = allele_value.clone();
                            Some(Chromosome::new(genes))
                        }
                    })
            })
            .collect::<Vec<_>>()
            .into()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.allele_list_sizes.iter().map(|v| *v - 1).sum::<usize>())
    }
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> PermutableGenotype for MultiDiscrete<T> {
    //noop
    fn allele_list_for_chromosome_permutations(&self) -> Vec<Self::Allele> {
        vec![]
    }

    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self>> + 'a> {
        Box::new(
            self.allele_lists
                .clone()
                .into_iter()
                .multi_cartesian_product()
                .map(Chromosome::new),
        )
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        self.allele_list_sizes
            .iter()
            .map(|v| BigUint::from(*v))
            .product()
    }
}

impl<T: PartialEq + Clone + Send + std::fmt::Debug> fmt::Display for MultiDiscrete<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        //writeln!(f, "  allele_lists: {:?}", self.allele_lists)?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size()
        )?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
