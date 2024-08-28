use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

/// Crossover with 50% probability for each gene to come from one of the two parents.
/// Optionally keep parents around to compete with children later on.
///
/// Actually implemented as `CrossoverMultiGene::new(<genes_size> / 2, keep_parent)`
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct Uniform {
    pub keep_parent: bool,
}
impl Crossover for Uniform {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        if state.population.size() < 2 {
            return;
        }
        let mut parent_chromosomes = if self.keep_parent {
            state.population.chromosomes.clone()
        } else {
            vec![] // throwaway to keep compiler happy
        };

        let number_of_crossovers = genotype.genes_size() / 2;
        for chunk in state.population.chromosomes.chunks_mut(2) {
            if let [father, mother] = chunk {
                genotype.crossover_chromosome_pair_multi_gene(
                    number_of_crossovers,
                    true,
                    father,
                    mother,
                    rng,
                );
            }
        }

        if self.keep_parent {
            state.population.chromosomes.append(&mut parent_chromosomes);
        }
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
}

impl Uniform {
    pub fn new(keep_parent: bool) -> Self {
        Self {
            keep_parent,
        }
    }
}
