use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use num::BigUint;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;
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
    pub seed_genes: Option<Vec<BinaryAllele>>,
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
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl Genotype for Binary {
    type Allele = BinaryAllele;
    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let genes: Vec<Self::Allele> = (0..self.genes_size)
                .map(|_| self.allele_sampler.sample(rng))
                .collect();
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome_random<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        let index = self.gene_index_sampler.sample(rng);
        chromosome.genes[index] = !chromosome.genes[index];
        chromosome.taint_fitness_score();
    }
}

impl IncrementalGenotype for Binary {
    fn mutate_chromosome_neighbour<R: Rng>(
        &self,
        chromosome: &mut Chromosome<Self>,
        _scale: Option<f32>,
        rng: &mut R,
    ) {
        self.mutate_chromosome_random(chromosome, rng);
    }

    fn chromosome_neighbours(
        &self,
        chromosome: &Chromosome<Self>,
        _scale: Option<f32>,
    ) -> Vec<Chromosome<Self>> {
        (0..self.genes_size)
            .map(|index| {
                let mut genes = chromosome.genes.clone();
                genes[index] = !genes[index];
                Chromosome::new(genes)
            })
            .collect()
    }

    fn chromosome_neighbours_size(&self) -> BigUint {
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
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
