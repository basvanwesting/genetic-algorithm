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
    allele_value_sampler: Uniform<ContinuousAllele>,
    allele_neighbour_sampler: Option<Uniform<ContinuousAllele>>,
    pub seed_genes: Option<Vec<ContinuousAllele>>,
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
                genes_size: genes_size,
                allele_range: allele_range.clone(),
                allele_neighbour_range: builder.allele_neighbour_range.clone(),
                gene_index_sampler: Uniform::from(0..genes_size),
                allele_value_sampler: Uniform::from(allele_range.clone()),
                allele_neighbour_sampler: builder
                    .allele_neighbour_range
                    .map(|allele_neighbour_range| Uniform::from(allele_neighbour_range.clone())),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for Continuous {
    type Allele = ContinuousAllele;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Allele> = (0..self.genes_size)
                .map(|_| self.allele_value_sampler.sample(rng))
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome_random<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = self.allele_value_sampler.sample(rng);
        chromosome.taint_fitness_score();
    }
}

impl IncrementalGenotype for Continuous {
    fn mutate_chromosome_neighbour<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        let new_value =
            chromosome.genes[index] + self.allele_neighbour_sampler.as_ref().unwrap().sample(rng);
        if new_value < self.allele_range.start {
            chromosome.genes[index] = self.allele_range.start;
        } else if new_value > self.allele_range.end {
            chromosome.genes[index] = self.allele_range.end;
        } else {
            chromosome.genes[index] = new_value;
        }
        chromosome.taint_fitness_score();
    }

    fn chromosome_neighbours(
        &self,
        chromosome: &Chromosome<Self>,
        scale: f32,
    ) -> Vec<Chromosome<Self>> {
        let diffs: Vec<ContinuousAllele> = vec![
            self.allele_neighbour_range.as_ref().unwrap().start * scale,
            self.allele_neighbour_range.as_ref().unwrap().end * scale,
        ]
        .into_iter()
        .dedup()
        .filter(|diff| *diff != 0.0)
        .collect();

        (0..self.genes_size())
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
            .collect()
    }

    fn chromosome_neighbours_size(&self) -> BigUint {
        let diffs: Vec<ContinuousAllele> = vec![
            self.allele_neighbour_range.as_ref().unwrap().start,
            self.allele_neighbour_range.as_ref().unwrap().end,
        ]
        .into_iter()
        .dedup()
        .filter(|diff| *diff != 0.0)
        .collect();

        BigUint::from(diffs.len() * self.genes_size())
    }

    fn chromosome_neighbour_permutations(
        &self,
        chromosome: &Chromosome<Self>,
        scale: f32,
    ) -> Vec<Chromosome<Self>> {
        let diffs: Vec<ContinuousAllele> = vec![
            self.allele_neighbour_range.as_ref().unwrap().start * scale,
            0.0,
            self.allele_neighbour_range.as_ref().unwrap().end * scale,
        ]
        .into_iter()
        .dedup()
        .collect();

        chromosome
            .genes
            .iter()
            .map(|gene| diffs.iter().map(|d| *gene + *d))
            .multi_cartesian_product()
            .map(|genes| {
                genes
                    .into_iter()
                    .map(|gene| {
                        if gene < self.allele_range.start {
                            self.allele_range.start
                        } else if gene > self.allele_range.end {
                            self.allele_range.end
                        } else {
                            gene
                        }
                    })
                    .collect()
            })
            .map(|genes| Chromosome::new(genes))
            .collect()
    }

    fn chromosome_neighbour_permutations_size(&self) -> BigUint {
        let diffs: Vec<ContinuousAllele> = vec![
            self.allele_neighbour_range.as_ref().unwrap().start,
            0.0,
            self.allele_neighbour_range.as_ref().unwrap().end,
        ]
        .into_iter()
        .dedup()
        .collect();

        BigUint::from(diffs.len()).pow(self.genes_size() as u32)
    }
}

impl fmt::Display for Continuous {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  allele_range: {:?}", self.allele_range)?;
        writeln!(f, "  chromosome_permutations_size: uncountable")?;
        writeln!(
            f,
            "  chromosome_neighbours_size: {}",
            self.chromosome_neighbours_size()
        )?;
        writeln!(
            f,
            "  chromosome_neighbour_permutations_size: {}",
            self.chromosome_neighbour_permutations_size()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
