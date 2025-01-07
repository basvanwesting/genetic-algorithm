use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, PermutateGenotype};
use crate::chromosome::{BinaryChromosome, Chromosome, ChromosomeManager, GenesOwner};
use crate::population::Population;
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Standard, Uniform, WeightedIndex};
use rand::prelude::*;
use std::fmt;

/// Genes are a vector of booleans. On random initialization, each gene has a 50% probability of
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
    pub mutation_gene_index_weights: Option<Vec<f64>>,
    pub crossover_gene_index_weights: Option<Vec<f64>>,
    pub crossover_point_index_weights: Option<Vec<f64>>,
    default_gene_index_sampler: Uniform<usize>,
    mutation_gene_index_sampler: Option<WeightedIndex<f64>>,
    crossover_gene_index_sampler: Option<WeightedIndex<f64>>,
    crossover_point_index_sampler: Option<WeightedIndex<f64>>,
    pub seed_genes_list: Vec<Vec<bool>>,
    pub chromosome_bin: Vec<BinaryChromosome>,
    pub best_genes: Vec<bool>,
}

impl TryFrom<Builder<Self>> for Binary {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if !builder.genes_size.is_some_and(|x| x > 0) {
            Err(TryFromBuilderError(
                "BinaryGenotype requires a genes_size > 0",
            ))
        } else {
            let genes_size = builder.genes_size.unwrap();
            Ok(Self {
                genes_size,
                default_gene_index_sampler: Uniform::from(0..genes_size),
                mutation_gene_index_sampler: builder
                    .mutation_gene_index_weights
                    .as_ref()
                    .map(|w| WeightedIndex::new(w.clone()).unwrap()),
                crossover_gene_index_sampler: builder
                    .crossover_gene_index_weights
                    .as_ref()
                    .map(|w| WeightedIndex::new(w.clone()).unwrap()),
                crossover_point_index_sampler: builder
                    .crossover_point_index_weights
                    .as_ref()
                    .map(|w| WeightedIndex::new(w.clone()).unwrap()),
                mutation_gene_index_weights: builder.mutation_gene_index_weights,
                crossover_gene_index_weights: builder.crossover_gene_index_weights,
                crossover_point_index_weights: builder.crossover_point_index_weights,
                seed_genes_list: builder.seed_genes_list,
                chromosome_bin: vec![],
                best_genes: vec![false; genes_size],
            })
        }
    }
}

impl Genotype for Binary {
    type Allele = bool;
    type Genes = Vec<Self::Allele>;
    type Chromosome = BinaryChromosome;

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

    fn mutate_chromosome_genes<R: Rng>(
        &mut self,
        number_of_mutations: usize,
        allow_duplicates: bool,
        chromosome: &mut Self::Chromosome,
        _scale_index: Option<usize>,
        rng: &mut R,
    ) {
        match (
            allow_duplicates,
            &self.mutation_gene_index_weights,
            &self.mutation_gene_index_sampler,
        ) {
            (true, _, Some(mutation_gene_index_sampler)) => {
                rng.sample_iter(mutation_gene_index_sampler)
                    .take(number_of_mutations)
                    .for_each(|index| {
                        chromosome.genes[index] = !chromosome.genes[index];
                    });
            }
            (true, _, _) => {
                rng.sample_iter(&self.default_gene_index_sampler)
                    .take(number_of_mutations)
                    .for_each(|index| {
                        chromosome.genes[index] = !chromosome.genes[index];
                    });
            }
            (false, Some(mutation_gene_index_weights), _) => {
                rand::seq::index::sample_weighted(
                    rng,
                    self.genes_size,
                    |i| mutation_gene_index_weights[i],
                    number_of_mutations.min(self.genes_size),
                )
                .unwrap()
                .iter()
                .for_each(|index| {
                    chromosome.genes[index] = !chromosome.genes[index];
                });
            }
            (false, _, _) => {
                rand::seq::index::sample(
                    rng,
                    self.genes_size,
                    number_of_mutations.min(self.genes_size),
                )
                .iter()
                .for_each(|index| {
                    chromosome.genes[index] = !chromosome.genes[index];
                });
            }
        }
        chromosome.taint();
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

impl EvolveGenotype for Binary {
    fn crossover_chromosome_genes<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
        rng: &mut R,
    ) {
        match (
            allow_duplicates,
            &self.crossover_gene_index_weights,
            &self.crossover_gene_index_sampler,
        ) {
            (true, _, Some(crossover_gene_index_sampler)) => {
                rng.sample_iter(crossover_gene_index_sampler)
                    .take(number_of_crossovers)
                    .for_each(|index| {
                        std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
                    });
            }
            (true, _, _) => {
                rng.sample_iter(self.default_gene_index_sampler)
                    .take(number_of_crossovers)
                    .for_each(|index| {
                        std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
                    });
            }
            (false, Some(crossover_gene_index_weights), _) => {
                rand::seq::index::sample_weighted(
                    rng,
                    self.genes_size,
                    |i| crossover_gene_index_weights[i],
                    number_of_crossovers.min(self.genes_size),
                )
                .unwrap()
                .iter()
                .for_each(|index| {
                    std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
                });
            }
            (false, _, _) => {
                rand::seq::index::sample(
                    rng,
                    self.genes_size(),
                    number_of_crossovers.min(self.genes_size()),
                )
                .iter()
                .for_each(|index| {
                    std::mem::swap(&mut father.genes[index], &mut mother.genes[index]);
                });
            }
        }
        mother.taint();
        father.taint();
    }
    fn crossover_chromosome_points<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
        rng: &mut R,
    ) {
        let mut indexes: Vec<usize> = match (
            allow_duplicates,
            &self.crossover_point_index_weights,
            &self.crossover_point_index_sampler,
        ) {
            (true, _, Some(crossover_point_index_sampler)) => rng
                .sample_iter(crossover_point_index_sampler)
                .take(number_of_crossovers)
                .collect(),
            (true, _, _) => rng
                .sample_iter(self.default_gene_index_sampler)
                .take(number_of_crossovers)
                .collect(),
            (false, Some(crossover_point_index_weights), _) => rand::seq::index::sample_weighted(
                rng,
                self.genes_size,
                |i| crossover_point_index_weights[i],
                number_of_crossovers.min(self.genes_size),
            )
            .unwrap()
            .into_iter()
            .collect(),
            (false, _, _) => rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .into_iter()
            .collect(),
        };

        indexes.sort_unstable();
        indexes
            .into_iter()
            .dedup()
            .chunks(2)
            .into_iter()
            .for_each(|mut chunk| match (chunk.next(), chunk.next()) {
                (Some(start_index), Some(end_index)) => {
                    let mother_back = &mut mother.genes[start_index..end_index];
                    let father_back = &mut father.genes[start_index..end_index];
                    father_back.swap_with_slice(mother_back);
                }
                (Some(start_index), _) => {
                    let mother_back = &mut mother.genes[start_index..];
                    let father_back = &mut father.genes[start_index..];
                    father_back.swap_with_slice(mother_back);
                }
                _ => (),
            });

        mother.taint();
        father.taint();
    }

    fn has_crossover_indexes(&self) -> bool {
        true
    }
    fn has_crossover_points(&self) -> bool {
        true
    }
}
impl HillClimbGenotype for Binary {
    fn fill_neighbouring_population<R: Rng>(
        &mut self,
        chromosome: &Self::Chromosome,
        population: &mut Population<Self::Chromosome>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) {
        (0..self.genes_size).for_each(|index| {
            let mut new_chromosome = self.chromosome_constructor_from(chromosome);
            new_chromosome.genes[index] = !new_chromosome.genes[index];
            population.chromosomes.push(new_chromosome);
        });
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.genes_size)
    }
}

impl PermutateGenotype for Binary {
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Self::Chromosome> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            Box::new(
                (0..self.genes_size())
                    .map(|_| vec![true, false])
                    .multi_cartesian_product()
                    .map(BinaryChromosome::new),
            )
        } else {
            Box::new(
                self.seed_genes_list
                    .clone()
                    .into_iter()
                    .map(BinaryChromosome::new),
            )
        }
    }
    fn chromosome_permutations_size(&self) -> BigUint {
        if self.seed_genes_list.is_empty() {
            BigUint::from(2u8).pow(self.genes_size() as u32)
        } else {
            self.seed_genes_list.len().into()
        }
    }
}

impl ChromosomeManager<Self> for Binary {
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> Vec<bool> {
        if self.seed_genes_list.is_empty() {
            rng.sample_iter(Standard).take(self.genes_size).collect()
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut BinaryChromosome, rng: &mut R) {
        chromosome.genes.clone_from(&self.random_genes_factory(rng));
    }
    fn copy_genes(&mut self, source: &BinaryChromosome, target: &mut BinaryChromosome) {
        target.genes.clone_from(&source.genes);
    }
    fn chromosome_bin_push(&mut self, chromosome: BinaryChromosome) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_find_or_create(&mut self) -> BinaryChromosome {
        self.chromosome_bin.pop().unwrap_or_else(|| {
            let genes = Vec::with_capacity(self.genes_size);
            BinaryChromosome::new(genes)
        })
    }
    fn chromosomes_cleanup(&mut self) {
        std::mem::take(&mut self.chromosome_bin);
    }
}

impl fmt::Display for Binary {
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
