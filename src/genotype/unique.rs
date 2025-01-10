use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, PermutateGenotype};
use crate::allele::Allele;
use crate::chromosome::{Chromosome, ChromosomeManager, GenesHash, GenesOwner, UniqueChromosome};
use crate::population::Population;
use factorial::Factorial;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

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
/// impl Allele for Item {}
///
/// let genotype = UniqueGenotype::builder()
///     .with_allele_list(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Unique<T: Allele + Hash = DefaultAllele> {
    pub genes_size: usize,
    pub allele_list: Vec<T>,
    gene_index_sampler: Uniform<usize>,
    pub seed_genes_list: Vec<Vec<T>>,
    pub chromosome_bin: Vec<UniqueChromosome<T>>,
    pub best_genes: Vec<T>,
    pub genes_hashing: bool,
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
                chromosome_bin: vec![],
                best_genes: allele_list.clone(),
                genes_hashing: builder.genes_hashing,
            })
        }
    }
}

impl<T: Allele + Hash> Genotype for Unique<T> {
    type Allele = T;
    type Genes = Vec<Self::Allele>;
    type Chromosome = UniqueChromosome<Self::Allele>;

    fn genes_size(&self) -> usize {
        self.genes_size
    }
    fn save_best_genes(&mut self, chromosome: &Self::Chromosome) {
        self.best_genes.clone_from(&chromosome.genes);
    }
    fn load_best_genes(&mut self, chromosome: &mut Self::Chromosome) {
        chromosome.genes.clone_from(&self.best_genes);
    }
    fn best_genes(&self) -> &Self::Genes {
        &self.best_genes
    }
    fn best_genes_slice(&self) -> &[Self::Allele] {
        self.best_genes.as_slice()
    }
    fn genes_slice<'a>(&'a self, chromosome: &'a Self::Chromosome) -> &'a [Self::Allele] {
        chromosome.genes.as_slice()
    }
    fn calculate_genes_hash(&self, chromosome: &Self::Chromosome) -> Option<GenesHash> {
        if self.genes_hashing {
            let mut s = DefaultHasher::new();
            chromosome.genes.hash(&mut s);
            Some(s.finish())
        } else {
            None
        }
    }

    fn mutate_chromosome_genes<R: Rng>(
        &mut self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Self::Chromosome,
        _scale_index: Option<usize>,
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
        chromosome.taint(self.calculate_genes_hash(chromosome));
    }
    fn set_seed_genes_list(&mut self, seed_genes_list: Vec<Self::Genes>) {
        self.seed_genes_list = seed_genes_list;
    }
    fn seed_genes_list(&self) -> &Vec<Self::Genes> {
        &self.seed_genes_list
    }
    fn max_scale_index(&self) -> Option<usize> {
        None
    }
}

impl<T: Allele + Hash> EvolveGenotype for Unique<T> {
    fn crossover_chromosome_genes<R: Rng>(
        &mut self,
        _number_of_crossovers: usize,
        _allow_duplicates: bool,
        _father: &mut Self::Chromosome,
        _mother: &mut Self::Chromosome,
        _rng: &mut R,
    ) {
        panic!("UniqueGenotype does not support gene crossover")
    }
    fn crossover_chromosome_points<R: Rng>(
        &mut self,
        _number_of_crossovers: usize,
        _allow_duplicates: bool,
        _father: &mut Self::Chromosome,
        _mother: &mut Self::Chromosome,
        _rng: &mut R,
    ) {
        panic!("UniqueGenotype does not support point crossover")
    }
}
impl<T: Allele + Hash> HillClimbGenotype for Unique<T> {
    fn fill_neighbouring_population<R: Rng>(
        &mut self,
        chromosome: &Self::Chromosome,
        population: &mut Population<Self::Chromosome>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) {
        (0..self.genes_size())
            .tuple_combinations()
            .for_each(|(first, second)| {
                let mut new_chromosome = self.chromosome_cloner(chromosome);
                new_chromosome.genes.swap(first, second);
                new_chromosome.taint(self.calculate_genes_hash(&new_chromosome));
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
    ) -> Box<dyn Iterator<Item = Self::Chromosome> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            Box::new(
                self.allele_list
                    .clone()
                    .into_iter()
                    .permutations(self.genes_size())
                    .map(UniqueChromosome::new),
            )
        } else {
            Box::new(
                self.seed_genes_list
                    .clone()
                    .into_iter()
                    .map(UniqueChromosome::new),
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
}

impl<T: Allele + Hash> ChromosomeManager<Self> for Unique<T> {
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<T> {
        if self.seed_genes_list.is_empty() {
            let mut genes = self.allele_list.clone();
            genes.shuffle(rng);
            genes
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut UniqueChromosome<T>, rng: &mut R) {
        chromosome.genes.clone_from(&self.random_genes_factory(rng));
    }
    fn copy_genes(&mut self, source: &UniqueChromosome<T>, target: &mut UniqueChromosome<T>) {
        target.genes.clone_from(&source.genes);
    }
    fn chromosome_bin_push(&mut self, chromosome: UniqueChromosome<T>) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_find_or_create(&mut self) -> UniqueChromosome<T> {
        self.chromosome_bin.pop().unwrap_or_else(|| {
            let genes = Vec::with_capacity(self.genes_size);
            UniqueChromosome::new(genes)
        })
    }
    fn chromosomes_cleanup(&mut self) {
        std::mem::take(&mut self.chromosome_bin);
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
            self.chromosome_permutations_size()
        )?;
        writeln!(
            f,
            "  neighbouring_population_size: {}",
            self.neighbouring_population_size()
        )?;
        writeln!(
            f,
            "  expected_number_of_sampled_index_duplicates: {}",
            self.expected_number_of_sampled_index_duplicates_report()
        )?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes_list.len())
    }
}
