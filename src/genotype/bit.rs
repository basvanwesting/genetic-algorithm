use super::builder::{Builder, TryFromBuilderError};
use super::{EvolveGenotype, Genotype, HillClimbGenotype, PermutateGenotype};
use crate::chromosome::{BitChromosome, ChromosomeManager, GenesHash, GenesOwner};
use crate::population::Population;
use fixedbitset::{Block, FixedBitSet};
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Standard, Uniform};
use rand::prelude::*;
use rustc_hash::FxHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Genes are a [FixedBitSet]. Functionally the same as [BinaryGenotype](super::BinaryGenotype), but
/// better for large genes sizes as storage is much more efficient than `Vec<bool>`.
///
/// Crossover points are limited to the [Block] size of [FixedBitSet] implementation (so only each 32
/// or 64 bits, but really fast). Crossover genes are not limited, but have the standard per bit
/// manipulation (slower). Keeping parents around during crossover is also much cheaper, due to the
/// reduced cloning cost.
///
/// On random initialization, each gene has a 50% probability of becoming true or false. Each gene
/// has an equal probability of mutating. If a gene mutates, its value is flipped.
///
/// # Example:
/// ```
/// use genetic_algorithm::genotype::{Genotype, BitGenotype};
///
/// let genotype = BitGenotype::builder()
///     .with_genes_size(10000)
///     .with_genes_hashing(false) // optional, defaults to false
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Bit {
    pub genes_size: usize,
    gene_index_sampler: Uniform<usize>,
    pub crossover_points: Vec<usize>,
    crossover_point_index_sampler: Option<Uniform<usize>>,
    pub seed_genes_list: Vec<FixedBitSet>,
    pub chromosome_bin: Vec<BitChromosome>,
    pub best_genes: FixedBitSet,
    pub genes_hashing: bool,
}

impl TryFrom<Builder<Self>> for Bit {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if !builder.genes_size.is_some_and(|x| x > 0) {
            Err(TryFromBuilderError("BitGenotype requires a genes_size > 0"))
        } else {
            let genes_size = builder.genes_size.unwrap();
            let mut crossover_points: Vec<usize> =
                (0..genes_size).step_by(Block::BITS as usize).collect();
            crossover_points.remove(0);
            let crossover_point_index_sampler = if crossover_points.is_empty() {
                None
            } else {
                Some(Uniform::from(0..crossover_points.len()))
            };
            Ok(Self {
                genes_size,
                gene_index_sampler: Uniform::from(0..builder.genes_size.unwrap()),
                crossover_points,
                crossover_point_index_sampler,
                seed_genes_list: builder.seed_genes_list,
                chromosome_bin: vec![],
                best_genes: FixedBitSet::default(),
                genes_hashing: builder.genes_hashing,
            })
        }
    }
}

impl Bit {
    /// ```
    /// use genetic_algorithm::genotype::BitGenotype;
    ///
    /// let genes = BitGenotype::genes_from_bools(vec![true, false, true, false, true]);
    /// assert_eq!(format!("{:b}", genes), "10101");
    /// ```
    pub fn genes_from_bools(bools: Vec<bool>) -> FixedBitSet {
        let mut bits = FixedBitSet::with_capacity(bools.len());
        bools.iter().enumerate().for_each(|(i, &b)| {
            bits.set(i, b);
        });
        bits
    }

    /// ```
    /// use genetic_algorithm::genotype::BitGenotype;
    ///
    /// let genes = BitGenotype::genes_from_str("10101");
    /// assert_eq!(format!("{:b}", genes), "10101");
    /// ```
    pub fn genes_from_str(str: &str) -> FixedBitSet {
        let mut bits = FixedBitSet::with_capacity(str.len());
        str.chars().enumerate().for_each(|(i, b)| match b {
            '1' => bits.insert(i),
            _ => bits.remove(i),
        });
        bits
    }
    /// ```
    /// use genetic_algorithm::genotype::BitGenotype;
    ///
    /// // block data beyond number if bits is ignored
    /// let genes = BitGenotype::genes_from_blocks(10, [usize::MAX, 1, 2]);
    /// assert_eq!(format!("{:b}", genes), "1111111111");
    ///
    /// let genes = BitGenotype::genes_from_blocks(100, [usize::MAX, 1, 2]);
    /// assert_eq!(genes.as_slice(), [usize::MAX, 1]);
    /// ```
    pub fn genes_from_blocks<I: IntoIterator<Item = Block>>(bits: usize, blocks: I) -> FixedBitSet {
        FixedBitSet::with_capacity_and_blocks(bits, blocks)
    }
}

impl Genotype for Bit {
    type Allele = Block;
    type Genes = FixedBitSet;
    type Chromosome = BitChromosome;

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
    fn genes_hashing(&self) -> bool {
        self.genes_hashing
    }
    fn calculate_genes_hash(&self, chromosome: &Self::Chromosome) -> Option<GenesHash> {
        if self.genes_hashing {
            let mut s = FxHasher::default();
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
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_mutations)
                .for_each(|index| chromosome.genes.toggle(index));
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size,
                number_of_mutations.min(self.genes_size),
            )
            .iter()
            .for_each(|index| chromosome.genes.toggle(index));
        }
        self.reset_chromosome_state(chromosome);
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

impl EvolveGenotype for Bit {
    fn crossover_chromosome_genes<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.gene_index_sampler)
                .take(number_of_crossovers)
                .for_each(|index| {
                    match (father.genes.contains(index), mother.genes.contains(index)) {
                        (true, false) => {
                            father.genes.remove(index);
                            mother.genes.insert(index);
                        }
                        (false, true) => {
                            father.genes.insert(index);
                            mother.genes.remove(index);
                        }
                        _ => (),
                    }
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.genes_size(),
                number_of_crossovers.min(self.genes_size()),
            )
            .iter()
            .for_each(|index| {
                match (father.genes.contains(index), mother.genes.contains(index)) {
                    (true, false) => {
                        father.genes.remove(index);
                        mother.genes.insert(index);
                    }
                    (false, true) => {
                        father.genes.insert(index);
                        mother.genes.remove(index);
                    }
                    _ => (),
                }
            });
        }
        self.reset_chromosome_state(mother);
        self.reset_chromosome_state(father);
    }
    fn crossover_chromosome_points<R: Rng>(
        &mut self,
        number_of_crossovers: usize,
        allow_duplicates: bool,
        father: &mut Self::Chromosome,
        mother: &mut Self::Chromosome,
        rng: &mut R,
    ) {
        if allow_duplicates {
            rng.sample_iter(self.crossover_point_index_sampler.unwrap())
                .take(number_of_crossovers)
                .for_each(|index| {
                    let mother_back = &mut mother.genes.as_mut_slice()[index..];
                    let father_back = &mut father.genes.as_mut_slice()[index..];
                    father_back.swap_with_slice(mother_back);
                });
        } else {
            rand::seq::index::sample(
                rng,
                self.crossover_points.len(),
                number_of_crossovers.min(self.crossover_points.len()),
            )
            .iter()
            .sorted_unstable()
            .chunks(2)
            .into_iter()
            .for_each(|mut chunk| match (chunk.next(), chunk.next()) {
                (Some(start_point_index), Some(end_point_index)) => {
                    let mother_back =
                        &mut mother.genes.as_mut_slice()[start_point_index..end_point_index];
                    let father_back =
                        &mut father.genes.as_mut_slice()[start_point_index..end_point_index];
                    father_back.swap_with_slice(mother_back);
                }
                (Some(start_point_index), _) => {
                    let mother_back = &mut mother.genes.as_mut_slice()[start_point_index..];
                    let father_back = &mut father.genes.as_mut_slice()[start_point_index..];
                    father_back.swap_with_slice(mother_back);
                }
                _ => (),
            });
        }
        self.reset_chromosome_state(mother);
        self.reset_chromosome_state(father);
    }

    fn has_crossover_indexes(&self) -> bool {
        true
    }
    fn has_crossover_points(&self) -> bool {
        true
    }
}
impl HillClimbGenotype for Bit {
    fn fill_neighbouring_population<R: Rng>(
        &mut self,
        chromosome: &Self::Chromosome,
        population: &mut Population<Self::Chromosome>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) {
        (0..self.genes_size).for_each(|index| {
            let mut new_chromosome = self.chromosome_cloner(chromosome);
            new_chromosome.genes.toggle(index);
            self.reset_chromosome_state(&mut new_chromosome);
            population.chromosomes.push(new_chromosome);
        });
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.genes_size)
    }
}

impl PermutateGenotype for Bit {
    fn chromosome_permutations_into_iter<'a>(
        &'a self,
        _chromosome: Option<&Self::Chromosome>,
        _scale_index: Option<usize>,
    ) -> Box<dyn Iterator<Item = Self::Chromosome> + Send + 'a> {
        if self.seed_genes_list.is_empty() {
            Box::new(
                (0..self.genes_size())
                    .map(|_| vec![true, false])
                    .multi_cartesian_product()
                    .map(Bit::genes_from_bools)
                    .map(BitChromosome::new),
            )
        } else {
            Box::new(
                self.seed_genes_list
                    .clone()
                    .into_iter()
                    .map(BitChromosome::new),
            )
        }
    }
    fn chromosome_permutations_size(&self, _scale_index: Option<usize>) -> BigUint {
        if self.seed_genes_list.is_empty() {
            BigUint::from(2u8).pow(self.genes_size() as u32)
        } else {
            self.seed_genes_list.len().into()
        }
    }
}

impl ChromosomeManager<Self> for Bit {
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> FixedBitSet {
        if self.seed_genes_list.is_empty() {
            FixedBitSet::with_capacity_and_blocks(self.genes_size, rng.sample_iter(Standard))
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn set_genes(&mut self, chromosome: &mut BitChromosome, genes: &FixedBitSet) {
        chromosome.genes.clone_from(genes);
        self.reset_chromosome_state(chromosome);
    }
    fn get_genes(&self, chromosome: &BitChromosome) -> FixedBitSet {
        chromosome.genes.clone()
    }
    fn copy_genes(&mut self, source: &BitChromosome, target: &mut BitChromosome) {
        target.genes.clone_from(&source.genes);
        self.copy_chromosome_state(source, target);
    }
    fn chromosome_bin_push(&mut self, chromosome: BitChromosome) {
        self.chromosome_bin.push(chromosome);
    }
    fn chromosome_bin_find_or_create(&mut self) -> BitChromosome {
        self.chromosome_bin.pop().unwrap_or_else(|| {
            let genes = FixedBitSet::with_capacity(self.genes_size);
            BitChromosome::new(genes)
        })
    }
    fn chromosomes_cleanup(&mut self) {
        std::mem::take(&mut self.chromosome_bin);
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  genes_size: {}", self.genes_size)?;
        writeln!(f, "  mutation_type: {:?}", self.mutation_type())?;
        writeln!(
            f,
            "  chromosome_permutations_size: {}",
            self.chromosome_permutations_size(None)
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
