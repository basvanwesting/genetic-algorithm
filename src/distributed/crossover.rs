//! The crossover phase where two parent chromosomes create two children chromosomes. The
//! [selection](crate::select) phase determines the order the parent pairing (overall with fitter
//! first).
//!
//! The selection_rate is the fraction of parents which are selected for
//! reproduction. This selection adds offspring to the population, the other
//! parents do not. The population now grows by the added offspring, as the
//! parents are not replaced yet. Value should typically be between 0.4 and
//! 0.8. High values risk of premature convergence. Low values reduce diversity
//! if overused.
//!
//! The crossover_rate (or recombination-rate) is the fraction of selected parents to crossover,
//! the remaining parents just clone as offspring. Value should typically be between 0.5 and 0.8.
//! High values converge faster, but risk losing good solutions. Low values have poor exploration
//! and risk of premature convergence
//!
//! Normally the crossover adds children to the popluation, thus increasing the population_size
//! above the target_population_size. Selection will reduce this again in the next generation
mod clone;
mod multi_gene;
mod multi_point;
mod rejuvenate;
mod single_gene;
mod single_point;
mod uniform;
mod wrapper;

pub use self::clone::Clone as CrossoverClone;
pub use self::multi_gene::MultiGene as CrossoverMultiGene;
pub use self::multi_point::MultiPoint as CrossoverMultiPoint;
pub use self::rejuvenate::Rejuvenate as CrossoverRejuvenate;
pub use self::single_gene::SingleGene as CrossoverSingleGene;
pub use self::single_point::SinglePoint as CrossoverSinglePoint;
pub use self::uniform::Uniform as CrossoverUniform;
pub use self::wrapper::Wrapper as CrossoverWrapper;

use crate::distributed::chromosome::Chromosome;
use crate::distributed::genotype::EvolveGenotype;
use crate::distributed::strategy::evolve::{EvolveConfig, EvolveState};
use crate::distributed::strategy::StrategyReporter;
use rand::Rng;

pub trait Crossover: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

    /// Helper method to expand chromosome population by cloning existing chromosomes
    /// in a round-robin fashion. Used by crossover implementations to prepare
    /// the population for crossover operations.
    fn expand_chromosome_population<T: crate::distributed::allele::Allele>(
        &self,
        chromosomes: &mut Vec<Chromosome<T>>,
        amount: usize,
    ) {
        let modulo = chromosomes.len();
        for i in 0..amount {
            let chromosome = chromosomes[i % modulo].clone();
            chromosomes.push(chromosome);
        }
    }

    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_indexes(&self) -> bool {
        false
    }
    /// to guard against invalid Crossover strategies which break the internal consistency
    /// of the genes, unique genotypes can't simply exchange genes without gene duplication issues
    fn require_crossover_points(&self) -> bool {
        false
    }
}
