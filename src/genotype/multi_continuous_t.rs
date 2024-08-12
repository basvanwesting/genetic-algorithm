use super::builder::{Builder, TryFromBuilderError};
use super::{Allele, Genotype, IncrementalGenotype};
use crate::chromosome::Chromosome;
use itertools::Itertools;
use num::BigUint;
use num::Zero;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;
use std::ops::RangeInclusive;

pub type DefaultAllele = f32;

/// Genes are a list of f32, each individually taken from its own allele_range. The genes_size is
/// derived to be the allele_ranges length. On random initialization, each gene gets a value
/// from its own allele_range with a uniform probability. Each gene has a weighted probability of
/// mutating, depending on its allele_range size. If a gene mutates, a new values is taken from its
/// own allele_range with a uniform probability. Duplicate allele values are allowed. Defaults to usize
/// as item.
///
/// Optionally an allele_neighbour_ranges can be provided. When this is done the mutation is
/// restricted to modify the existing value by a difference taken from allele_neighbour_range with a uniform probability.
///
/// # Example:
/// ```
/// use genetic_algorithm::genotype::{Genotype, MultiContinuousGenotype};
///
/// let genotype = MultiContinuousGenotype::builder()
///     .with_allele_ranges(vec![
///        (0.0..=10.0),
///        (5.0..=20.0),
///        (0.0..=5.0),
///        (10.0..=30.0),
///     ])
///     .with_allele_neighbour_ranges(vec![
///        (-1.0..=1.0),
///        (-2.0..=2.0),
///        (-0.5..=0.5),
///        (-3.0..=3.0),
///     ]) // optional
///     .build()
///     .unwrap();
/// ```
pub struct MultiContinuous<
    T: Allele + Copy + Default + Zero + Into<f64> + std::ops::Add<Output = T> + std::cmp::PartialOrd = DefaultAllele,
> where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    genes_size: usize,
    pub allele_ranges: Vec<RangeInclusive<T>>,
    pub allele_neighbour_ranges: Option<Vec<RangeInclusive<T>>>,
    gene_index_sampler: WeightedIndex<f64>,
    allele_samplers: Vec<Uniform<T>>,
    allele_neighbour_samplers: Option<Vec<Uniform<T>>>,
    pub seed_genes_list: Vec<Vec<T>>,
}

impl<
        T: Allele
            + Copy
            + Default
            + Zero
            + Into<f64>
            + std::ops::Add<Output = T>
            + std::cmp::PartialOrd,
    > TryFrom<Builder<Self>> for MultiContinuous<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_ranges.is_none() {
            Err(TryFromBuilderError(
                "MultiContinuousGenotype requires a allele_ranges",
            ))
        } else if builder
            .allele_ranges
            .as_ref()
            .map(|o| o.is_empty())
            .unwrap()
        {
            Err(TryFromBuilderError(
                "MultiContinuousGenotype requires non-empty allele_ranges",
            ))
        } else {
            let allele_ranges = builder.allele_ranges.unwrap();
            let genes_size = allele_ranges.len();
            let index_weights: Vec<f64> = allele_ranges
                .iter()
                .map(|allele_range| (*allele_range.end()).into() - (*allele_range.start()).into())
                .collect();

            Ok(Self {
                genes_size,
                allele_ranges: allele_ranges.clone(),
                allele_neighbour_ranges: builder.allele_neighbour_ranges.clone(),
                gene_index_sampler: WeightedIndex::new(index_weights).unwrap(),
                allele_samplers: allele_ranges
                    .iter()
                    .map(|allele_range| Uniform::from(allele_range.clone()))
                    .collect(),
                allele_neighbour_samplers: builder.allele_neighbour_ranges.map(
                    |allele_neighbour_ranges| {
                        allele_neighbour_ranges
                            .iter()
                            .map(|allele_neighbour_range| {
                                Uniform::from(allele_neighbour_range.clone())
                            })
                            .collect()
                    },
                ),
                seed_genes_list: builder.seed_genes_list,
            })
        }
    }
}

impl<
        T: Allele
            + Copy
            + Default
            + Zero
            + Into<f64>
            + std::ops::Add<Output = T>
            + std::cmp::PartialOrd,
    > Genotype for MultiContinuous<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    type Allele = T;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele> {
        if self.seed_genes_list.is_empty() {
            (0..self.genes_size)
                .map(|index| self.allele_samplers[index].sample(rng))
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self::Allele> {
        Chromosome::new(self.random_genes_factory(rng))
    }

    fn mutate_chromosome_random<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    ) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.allele_samplers[index].sample(rng);
        chromosome.taint_fitness_score();
    }
    //FIXME: scale doesn't work for generic
    fn mutate_chromosome_neighbour<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        _scale: Option<f32>,
        rng: &mut R,
    ) {
        let index = self.gene_index_sampler.sample(rng);
        let allele_ranges = &self.allele_ranges[index];
        let new_value = chromosome.genes[index]
            + self.allele_neighbour_samplers.as_ref().unwrap()[index].sample(rng);
        if new_value < *allele_ranges.start() {
            chromosome.genes[index] = *allele_ranges.start();
        } else if new_value > *allele_ranges.end() {
            chromosome.genes[index] = *allele_ranges.end();
        } else {
            chromosome.genes[index] = new_value;
        }
        chromosome.taint_fitness_score();
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Vec<T>>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Vec<T>> {
        &self.seed_genes_list
    }
}

impl<
        T: Allele
            + Copy
            + Default
            + Zero
            + Into<f64>
            + std::ops::Add<Output = T>
            + std::cmp::PartialOrd,
    > IncrementalGenotype for MultiContinuous<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    //FIXME: scale doesn't work for generic
    fn neighbouring_chromosomes(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        _scale: Option<f32>,
    ) -> Vec<Chromosome<Self::Allele>> {
        let range_diffs: Vec<Vec<T>> = self
            .allele_neighbour_ranges
            .as_ref()
            .unwrap()
            .iter()
            .map(|range| vec![*range.start(), *range.end()])
            .map(|range| {
                range
                    .into_iter()
                    .dedup()
                    .filter(|diff| !diff.is_zero())
                    .collect()
            })
            .collect();

        self.allele_ranges
            .iter()
            .enumerate()
            .flat_map(|(index, value_range)| {
                range_diffs[index].iter().map(move |diff| {
                    let mut genes = chromosome.genes.clone();
                    let new_value = genes[index] + *diff;
                    if new_value < *value_range.start() {
                        genes[index] = *value_range.start();
                    } else if new_value > *value_range.end() {
                        genes[index] = *value_range.end();
                    } else {
                        genes[index] = new_value;
                    }
                    Chromosome::new(genes)
                })
            })
            .collect::<Vec<_>>()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(2 * self.genes_size)
    }
}

impl<
        T: Allele
            + Copy
            + Default
            + Zero
            + Into<f64>
            + std::ops::Add<Output = T>
            + std::cmp::PartialOrd,
    > Clone for MultiContinuous<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            genes_size: self.genes_size.clone(),
            allele_ranges: self.allele_ranges.clone(),
            allele_neighbour_ranges: self.allele_neighbour_ranges.clone(),
            gene_index_sampler: self.gene_index_sampler.clone(),
            allele_samplers: self
                .allele_ranges
                .iter()
                .map(|allele_range| Uniform::from(allele_range.clone()))
                .collect(),
            allele_neighbour_samplers: self.allele_neighbour_ranges.clone().map(
                |allele_neighbour_ranges| {
                    allele_neighbour_ranges
                        .iter()
                        .map(|allele_neighbour_range| Uniform::from(allele_neighbour_range.clone()))
                        .collect()
                },
            ),
            seed_genes_list: self.seed_genes_list.clone(),
        }
    }
}

impl<
        T: Allele
            + Copy
            + Default
            + Zero
            + Into<f64>
            + std::ops::Add<Output = T>
            + std::cmp::PartialOrd,
    > fmt::Debug for MultiContinuous<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("genes_size", &self.genes_size)
            .field("allele_ranges", &self.allele_ranges)
            .field("allele_neighbour_ranges", &self.allele_neighbour_ranges)
            .field("gene_index_sampler", &self.gene_index_sampler)
            .field("seed_genes_list", &self.seed_genes_list)
            .finish()
    }
}

impl<
        T: Allele
            + Copy
            + Default
            + Zero
            + Into<f64>
            + std::ops::Add<Output = T>
            + std::cmp::PartialOrd,
    > fmt::Display for MultiContinuous<T>
where
    T: SampleUniform,
    Uniform<T>: Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  allele_ranges: {:?}", self.allele_ranges)?;
        writeln!(
            f,
            "  allele_neighbour_ranges: {:?}",
            self.allele_neighbour_ranges
        )?;
        writeln!(f, "  chromosome_permutations_size: uncountable")?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size()
        )?;
        writeln!(f, "  seed_genes_list: {:?}", self.seed_genes_list)
    }
}
