use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, MutationType, PermutateGenotype};
use crate::allele::Allele;
use crate::chromosome::{Chromosome, Genes};
use crate::population::Population;
use factorial::Factorial;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::fmt;
use std::hash::Hash;

pub type DefaultAllele = usize;

/// Genes are a vector of unique values, taken from the allele_list using clone(), each value occurs
/// exactly once. The genes_size is derived to be the same as allele_list length. On random
/// initialization, the allele_list are shuffled to form the genes. Each pair of genes has an equal
/// probability of mutating. If a pair of genes mutates, the values are switched, ensuring the list
/// of alleles remains unique. Defaults to usize as item.
///
/// # Panics
///
/// Does not support gene or point crossover. Will panic when tried, but
/// [EvolveBuilder](crate::strategy::evolve::EvolveBuilder) shouldn't allow this.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, UniqueGenotype};
///
/// let genotype = UniqueGenotype::builder()
///     .with_allele_list((0..100).collect())
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct)
/// ```
/// use genetic_algorithm::genotype::{Allele, Genotype, UniqueGenotype};
///
/// #[derive(Clone, Copy, PartialEq, Hash, Debug)]
/// struct Item(pub u16, pub u16);
/// genetic_algorithm::impl_allele!(Item);
///
/// let genotype = UniqueGenotype::builder()
///     .with_allele_list(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .with_genes_hashing(true) // optional, defaults to true
///     .with_chromosome_recycling(true) // optional, defaults to true
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Unique<T: Allele + Hash = DefaultAllele> {
    pub genes_size: usize,
    pub allele_list: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes_list: Vec<Vec<T>>,
    pub genes_hashing: bool,
    pub chromosome_recycling: bool,
}

impl<T: Allele + Hash> TryFrom<Builder<Self>> for Unique<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.allele_list.is_none() {
            Err(TryFromBuilderError("UniqueGenotype requires allele_list"))
        } else if builder.allele_list.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "UniqueGenotype requires non-empty allele_list",
            ))
        } else {
            let allele_list = builder.allele_list.unwrap();
            let genes_size = allele_list.len();
            Ok(Self {
                genes_size,
                allele_list: allele_list.clone(),
                gene_index_sampler: Uniform::from(0..allele_list.len()),
                seed_genes_list: builder.seed_genes_list,
                genes_hashing: builder.genes_hashing,
                chromosome_recycling: builder.chromosome_recycling,
            })
        }
    }
}

impl<T: Allele + Hash> Unique<T> {
    fn mutation_type(&self) -> MutationType {
        MutationType::Random
    }
}
impl<T: Allele + Hash> Genotype for Unique<T> {
    type Allele = T;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn genes_slice<'a>(&'a self, chromosome: &'a Chromosome<Self::Allele>) -> &'a [Self::Allele] {
        chromosome.genes.as_slice()
    }

    fn sample_gene_index<R: Rng>(&self, rng: &mut R) -> usize {
        self.gene_index_sampler.sample(rng)
    }
    fn sample_gene_indices<R: Rng>(
        &self,
        count: usize,
        allow_duplicates: bool,
        rng: &mut R,
    ) -> Vec<usize> {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(count)
                .collect()
        } else {
            rand::seq::index::sample(rng, self.genes_size, count.min(self.genes_size)).into_vec()
        }
    }

    fn mutate_chromosome_genes<R: Rng>(
        &self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Chromosome<Self::Allele>,
        rng: &mut R,
    ) {
        if allow_duplicates {
            for _ in 0..number_of_mutations {
                let index1 = self.gene_index_sampler.sample(rng);
                let index2 = self.gene_index_sampler.sample(rng);
                chromosome.genes.swap(index1, index2);
            }
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size,
                (number_of_mutations * 2).min(self.genes_size),
            )
            .iter()
            .tuples()
            .for_each(|(index1, index2)| chromosome.genes.swap(index1, index2));
        }
        chromosome.reset_metadata(self.genes_hashing);
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Genes<Self::Allele>>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Genes<Self::Allele>> {
        &self.seed_genes_list
    }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<T> {
        if self.seed_genes_list.is_empty() {
            let mut genes = self.allele_list.clone();
            genes.shuffle(rng);
            genes
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn genes_capacity(&self) -> usize {
        self.genes_size
    }
    fn genes_hashing(&self) -> bool {
        self.genes_hashing
    }
    fn chromosome_recycling(&self) -> bool {
        self.chromosome_recycling
    }
}

impl<T: Allele + Hash> EvolveGenotype for Unique<T> {
    fn crossover_chromosome_genes<R: Rng>(
        &self,
        _number_of_crossovers: usize,
        _allow_duplicates: bool,
        _father: &mut Chromosome<Self::Allele>,
        _mother: &mut Chromosome<Self::Allele>,
        _rng: &mut R,
    ) {
        panic!("UniqueGenotype does not support gene crossover")
    }
    fn crossover_chromosome_points<R: Rng>(
        &self,
        _number_of_crossovers: usize,
        _allow_duplicates: bool,
        _father: &mut Chromosome<Self::Allele>,
        _mother: &mut Chromosome<Self::Allele>,
        _rng: &mut R,
    ) {
        panic!("UniqueGenotype does not support point crossover")
    }
}
impl<T: Allele + Hash> HillClimbGenotype for Unique<T> {
    fn fill_neighbouring_population<R: Rng>(
        &self,
        chromosome: &Chromosome<Self::Allele>,
        population: &mut Population<Self::Allele>,
        _rng: &mut R,
    ) {
        (0..self.genes_size())
            .tuple_combinations()
            .for_each(|(first, second)| {
                let mut new_chromosome = population.new_chromosome(chromosome);
                new_chromosome.genes.swap(first, second);
                new_chromosome.reset_metadata(self.genes_hashing);
                population.chromosomes.push(new_chromosome);
            });
    }

    fn neighbouring_population_size(&self) -> BigUint {
        let n = BigUint::from(self.genes_size);
        let k = BigUint::from(2usize);

        n.factorial() / (k.factorial() * (n - k).factorial())
    }
}

impl<T: Allele + Hash> PermutateGenotype for Unique<T> {
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        _chromosome: Option<&Chromosome<Self::Allele>>,
    ) -> Box<dyn Iterator<Item = Chromosome<Self::Allele>> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            Box::new(
                self.allele_list
                    .clone()
                    .into_iter()
                    .permutations(self.genes_size())
                    .map(Chromosome::new),
            )
        } else {
            Box::new(
                self.seed_genes_list
                    .clone()
                    .into_iter()
                    .map(Chromosome::new),
            )
        }
    }

    fn chromosome_permutations_size(&self) -> BigUint {
        if self.seed_genes_list.is_empty() {
            BigUint::from(self.genes_size).factorial()
        } else {
            self.seed_genes_list.len().into()
        }
    }
    fn mutation_type_allows_permutation(&self) -> bool {
        true
    }
}

impl<T: Allele + Hash> fmt::Display for Unique<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type())?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size_report()
        )?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size_report()
        )?;
        writeln!(
            f,
            "  expected_number_of_sampled_index_duplicates: {}",
            self.expected_number_of_sampled_index_duplicates_report()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes_list.len())
    }
}
