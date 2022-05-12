use super::builder::{Builder, TryFromBuilderError};
use super::{Genotype, PermutableGenotype};
use crate::chromosome::Chromosome;
use factorial::Factorial;
use itertools::Itertools;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::prelude::*;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

pub type DefaultSetGene = usize;

/// Genes are a list of unordered unique values, taken from the gene_values using clone(), each
/// value occurs zero or one time. The gene_size is variable, between zero and the gene_values
/// length. On random initialization, a random number of gene_values is taken to form the genes. If
/// the genes mutate, there is a 50% probability of an existing value being dropped from the genes
/// or a 50% probability for a non-existing value to be added. Defaults to usize as item.
///
/// # Example (usize, default):
/// ```
/// use genetic_algorithm::genotype::{Genotype, SetGenotype};
///
/// let genotype = SetGenotype::builder()
///     .with_gene_values((0..100).collect())
///     .build()
///     .unwrap();
/// ```
///
/// # Example (struct)
/// ```
/// use genetic_algorithm::genotype::{Genotype, SetGenotype};
///
/// #[derive(Clone, Debug, Hash, Eq, PartialEq)]
/// struct Item(pub u16, pub u16);
///
/// let genotype = SetGenotype::builder()
///     .with_gene_values(vec![
///         Item(23, 505),
///         Item(26, 352),
///         Item(20, 458),
///     ])
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Set<T: Clone + std::fmt::Debug + Hash + Eq = DefaultSetGene> {
    pub gene_values: Vec<T>,
    gene_set: HashSet<T>,
    gene_value_index_sampler: Uniform<usize>,
    boolean_sampler: Bernoulli,
    pub seed_genes: Option<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug + Hash + Eq> TryFrom<Builder<Self>> for Set<T> {
    type Error = TryFromBuilderError;

    fn try_from(builder: Builder<Self>) -> Result<Self, Self::Error> {
        if builder.gene_values.is_none() {
            Err(TryFromBuilderError("SetGenotype requires gene_values"))
        } else if builder.gene_values.as_ref().map(|o| o.is_empty()).unwrap() {
            Err(TryFromBuilderError(
                "SetGenotype requires non-empty gene_values",
            ))
        } else {
            let gene_values = builder.gene_values.unwrap();
            let gene_set = HashSet::from_iter(gene_values.clone().into_iter());
            Ok(Self {
                gene_values: gene_values.clone(),
                gene_set: gene_set,
                gene_value_index_sampler: Uniform::from(0..gene_values.len()),
                boolean_sampler: Bernoulli::new(0.5).unwrap(),
                seed_genes: builder.seed_genes,
            })
        }
    }
}

impl<T: Clone + std::fmt::Debug + Hash + Eq> Genotype for Set<T> {
    type Gene = T;
    // TODO: more like max_gene_size()
    fn gene_size(&self) -> usize {
        self.gene_values.len()
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<Self> {
        if let Some(seed_genes) = self.seed_genes.as_ref() {
            Chromosome::new(seed_genes.clone())
        } else {
            let mut genes = self.gene_values.clone();
            genes.shuffle(rng);
            genes.truncate(self.gene_value_index_sampler.sample(rng));
            Chromosome::new(genes)
        }
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<Self>, rng: &mut R) {
        if chromosome.genes.len() > 0 && self.boolean_sampler.sample(rng) {
            // remove an item
            let index = rng.gen_range(0..chromosome.genes.len());
            chromosome.genes.swap_remove(index);
        } else {
            // add an item
            let difference_size = self.gene_values.len() - chromosome.genes.len();
            if difference_size == 0 {
                return;
            }
            let current_gene_set = HashSet::from_iter(chromosome.genes.clone().into_iter());
            let mut difference = self.gene_set.difference(&current_gene_set);
            let index = rng.gen_range(0..difference_size);
            chromosome
                .genes
                .push(difference.nth(index).unwrap().clone());
        }
        chromosome.taint_fitness_score();
    }

    fn is_unique(&self) -> bool {
        true
    }
}

impl<T: Clone + std::fmt::Debug + Hash + Eq> PermutableGenotype for Set<T> {
    fn gene_values(&self) -> Vec<Self::Gene> {
        self.gene_values.clone()
    }

    fn chromosome_permutations_into_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Chromosome<Self>> + 'a> {
        let n = self.gene_values.len();
        (0..n)
            .map(|r| {
                self.gene_values
                    .clone()
                    .into_iter()
                    .combinations(r)
                    .map(|genes| Chromosome::new(genes))
            })
            .fold(Box::new(None.into_iter()), |acc, c| {
                Box::new(acc.chain(Box::new(c)))
            })
    }

    fn chromosome_permutations_size(&self) -> usize {
        let n = self.gene_values.len();
        (0..n).fold(0, |acc, r| {
            acc + n.factorial() / (r.factorial() * (n - r).factorial())
        })
    }
}

impl<T: Clone + std::fmt::Debug + Hash + Eq> fmt::Display for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "genotype:")?;
        writeln!(f, "  gene_values: {:?}", self.gene_values)?;
        writeln!(f, "  seed_genes: {:?}", self.seed_genes)
    }
}
