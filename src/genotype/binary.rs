use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use num::BigUint;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::prelude::*;
use std::fmt;

pub type BinaryAllele = bool;

/// Genes are a list of booleans. On random initialization, each gene has a 50% probability of
/// becoming true or false. Each gene has an equal probability of mutating. If a gene mutates, its
/// value is flipped.
///
/// # Example:
/// ```
/// use genetic_algorithm::genotype::{Genotype, BinaryGenotype};
///
/// let genotype = BinaryGenotype::builder()
///     .with_genes_size(100)
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Binary {
    pub genes_size: usize,
    gene_index_sampler: Uniform<usize>,
    allele_sampler: Bernoulli,
    pub seed_genes_list: Vec<Vec<BinaryAllele>>,
}

impl TryFrom<Builder<Self>> for Binary {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.genes_size.is_none() {
            Err(TryFromBuilderError("BinaryGenotype requires a genes_size"))
        } else {
            Ok(Self {
                genes_size: builder.genes_size.unwrap(),
                gene_index_sampler: Uniform::from(0..builder.genes_size.unwrap()),
                allele_sampler: Bernoulli::new(0.5).unwrap(),
                seed_genes_list: builder.seed_genes_list,
            })
        }
    }
}

impl Genotype for Binary {
    type Allele = BinaryAllele;
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
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self::Allele> {
        Chromosome::new(self.random_genes_factory(rng))
    }

    fn mutate_chromosome<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self::Allele>,
        _scale_index: Option<usize>,
        rng: &mut R,
    ) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = !chromosome.genes[index];
        chromosome.taint_fitness_score();
    }

    fn crossover_index_sampler(&self) -> Option<&Uniform<usize>> {
        Some(&self.gene_index_sampler)
    }
    fn crossover_point_sampler(&self) -> Option<&Uniform<usize>> {
        Some(&self.gene_index_sampler)
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Vec<BinaryAllele>>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Vec<BinaryAllele>> {
        &self.seed_genes_list
    }
    fn max_scale_index(&self) -> Option<usize> {
        None
    }
}

impl IncrementalGenotype for Binary {
    fn neighbouring_chromosomes<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) -> Vec<Chromosome<Self::Allele>> {
        (0..self.genes_size)
            .map(|index| {
                let mut genes = chromosome.genes.clone();
                genes[index] = !genes[index];
                Chromosome::new(genes)
            })
            .collect::<Vec<_>>()
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.genes_size)
    }
}

impl PermutableGenotype for Binary {
    fn allele_list_for_chromosome_permutations(&self) -> Vec<Self::Allele> {
        vec![true, false]
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
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
        writeln!(f, "  seed_genes_list: {:?}", self.seed_genes_list)
    }
}
