use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, IncrementalGenotype, PermutableGenotype};
use crate::chromosome::{BitChromosome, Chromosome, ChromosomeManager, OwnsGenes};
use fixedbitset::{Block, FixedBitSet};
use itertools::Itertools;
use num::BigUint;
use rand::distributions::{Standard, Uniform};
use rand::prelude::*;
use std::fmt;

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
    pub chromosome_recycling: bool,
    pub chromosome_bin: Vec<BitChromosome>,
    pub best_genes: FixedBitSet,
}

impl TryFrom<Builder<Self>> for Bit {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.genes_size.is_none() {
            Err(TryFromBuilderError("BitGenotype requires a genes_size"))
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
                chromosome_recycling: builder.chromosome_recycling,
                chromosome_bin: vec![],
                best_genes: FixedBitSet::default(),
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
    type Allele = ();
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
        chromosome.taint();
    }

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
        mother.taint();
        father.taint();
    }

    fn has_crossover_indexes(&self) -> bool {
        true
    }
    fn has_crossover_points(&self) -> bool {
        true
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

impl IncrementalGenotype for Bit {
    fn fill_neighbouring_population<R: Rng>(
        &mut self,
        chromosome: &Self::Chromosome,
        output_chromosomes: &mut Vec<Self::Chromosome>,
        _scale_index: Option<usize>,
        _rng: &mut R,
    ) {
        (0..self.genes_size).for_each(|index| {
            let mut new_chromosome = self.chromosome_constructor_from(chromosome);
            new_chromosome.genes.toggle(index);
            output_chromosomes.push(new_chromosome);
        });
    }

    fn neighbouring_population_size(&self) -> BigUint {
        BigUint::from(self.genes_size)
    }
}

impl PermutableGenotype for Bit {
    fn chromosome_permutations_into_iter(&self) -> impl Iterator<Item = BitChromosome> + Send {
        (0..self.genes_size())
            .map(|_| vec![true, false])
            .multi_cartesian_product()
            .map(Bit::genes_from_bools)
            .map(BitChromosome::new)
    }
    fn chromosome_permutations_size(&self) -> BigUint {
        BigUint::from(2u8).pow(self.genes_size() as u32)
    }
}

impl ChromosomeManager<Self> for Bit {
    // fn chromosome_constructor<R: Rng>(&mut self, rng: &mut R) -> BitChromosome {
    //     BitChromosome::new(self.random_genes_factory(rng))
    // }
    fn random_genes_factory<R: Rng>(&self, rng: &mut R) -> FixedBitSet {
        if self.seed_genes_list.is_empty() {
            FixedBitSet::with_capacity_and_blocks(self.genes_size, rng.sample_iter(Standard))
        } else {
            self.seed_genes_list.choose(rng).unwrap().clone()
        }
    }
    fn set_random_genes<R: Rng>(&mut self, chromosome: &mut BitChromosome, rng: &mut R) {
        chromosome.genes.clone_from(&self.random_genes_factory(rng));
    }
    fn copy_genes(&mut self, source: &BitChromosome, target: &mut BitChromosome) {
        target.genes.clone_from(&source.genes);
    }
    fn chromosome_recycling(&self) -> bool {
        self.chromosome_recycling
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
}

impl fmt::Display for Bit {
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
