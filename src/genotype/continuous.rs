use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype};
use crate::chromosome::Chromosome;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;
use std::ops::Range;

pub type ContinuousAllele = f32;

/// Genes are a list of f32, each taken from the allele_range using clone(). On random initialization, each
/// gene gets a value from the allele_range with a uniform probability. Each gene has an equal probability
/// of mutating. If a gene mutates, a new value is taken from allele_range with a uniform probability.
///
/// Optionally an allele_neighbour_range can be provided. When this is done the mutation is
/// restricted to modify the existing value by a difference taken from allele_neighbour_range with a uniform probability.
///
/// # Example:
/// ```
/// use genetic_algorithm::genotype::{Genotype, ContinuousGenotype};
///
/// let genotype = ContinuousGenotype::builder()
///     .with_genes_size(100)
///     .with_allele_range(0.0..1.0)
///     .with_allele_neighbour_range(-0.1..0.1) // optional
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Continuous {
    pub genes_size: usize,
    pub allele_range: Range<ContinuousAllele>,
    pub allele_neighbour_range: Option<Range<ContinuousAllele>>,
    gene_index_sampler: Uniform<usize>,
    allele_sampler: Uniform<ContinuousAllele>,
    allele_neighbour_sampler: Option<Uniform<ContinuousAllele>>,
    pub seed_genes_list: Vec<Vec<ContinuousAllele>>,
}

impl TryFrom<Builder<Self>> for Continuous {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.genes_size.is_none() {
            Err(TryFromBuilderError(
                "ContinuousGenotype requires a genes_size",
            ))
        } else if builder.allele_range.is_none() {
            Err(TryFromBuilderError(
                "ContinuousGenotype requires a allele_range",
            ))
        } else {
            let genes_size = builder.genes_size.unwrap();
            let allele_range = builder.allele_range.unwrap();

            Ok(Self {
                genes_size,
                allele_range: allele_range.clone(),
                allele_neighbour_range: builder.allele_neighbour_range.clone(),
                gene_index_sampler: Uniform::from(0..genes_size),
                allele_sampler: Uniform::from(allele_range.clone()),
                allele_neighbour_sampler: builder
                    .allele_neighbour_range
                    .map(|allele_neighbour_range| Uniform::from(allele_neighbour_range.clone())),
                seed_genes_list: builder.seed_genes_list,
            })
        }
    }
}

impl Genotype for Continuous {
    type Allele = ContinuousAllele;
    fn genes_size(&self) -> usize {
        self.genes_size
    }

    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<Self::Allele> {
        if self.seed_genes_list.is_empty() {
            (0..self.genes_size)
                .map(|_| self.allele_sampler.sample(rng))
                .collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        Chromosome::new(self.random_genes_factory(rng))
    }

    fn mutate_chromosome_random<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.allele_sampler.sample(rng);
        chromosome.taint_fitness_score();
    }
    fn mutate_chromosome_neighbour<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self>,
        scale: Option<f32>,
        rng: &mut R,
    ) {
        let index = self.gene_index_sampler.sample(rng);
        let new_value = chromosome.genes[index]
            + self.allele_neighbour_sampler.as_ref().unwrap().sample(rng) * scale.unwrap_or(1.0);
        if new_value < self.allele_range.start {
            chromosome.genes[index] = self.allele_range.start;
        } else if new_value > self.allele_range.end {
            chromosome.genes[index] = self.allele_range.end;
        } else {
            chromosome.genes[index] = new_value;
        }
        chromosome.taint_fitness_score();
    }

    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Vec<ContinuousAllele>>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Vec<ContinuousAllele>> {
        &self.seed_genes_list
    }
}

impl IncrementalGenotype for Continuous {
    fn neighbouring_chromosomes(
        &self,
        chromosome: &Chromosome<Self>,
        scale: Option<f32>,
    ) -> Vec<Chromosome<Self>> {
        let diffs: Vec<ContinuousAllele> = vec![
            self.allele_neighbour_range.as_ref().unwrap().start * scale.unwrap_or(1.0),
            self.allele_neighbour_range.as_ref().unwrap().end * scale.unwrap_or(1.0),
        ]
        .into_iter()
        .dedup()
        .filter(|diff| *diff != 0.0)
        .collect();

        (0..self.genes_size)
            .flat_map(|index| {
                diffs.iter().map(move |diff| {
                    let mut genes = chromosome.genes.clone();
                    let new_value = genes[index] + *diff;
                    if new_value < self.allele_range.start {
                        genes[index] = self.allele_range.start;
                    } else if new_value > self.allele_range.end {
                        genes[index] = self.allele_range.end;
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

impl fmt::Display for Continuous {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  allele_range: {:?}", self.allele_range)?;
        writeln!(
            f,
            "  allele_neighbour_range: {:?}",
            self.allele_neighbour_range
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
