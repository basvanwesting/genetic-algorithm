use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

/// Crossover starting with clones of the parents, with a single gene taken from the other parent.
/// The single gene is chosen with uniform probability.
/// Optionally keep parents around to compete with children later on.
///
/// Not allowed for unique genotypes as it would not preserve the gene uniqueness in the children.
#[derive(Clone, Debug)]
pub struct SingleGene {
    pub keep_parent: bool,
}
impl Crossover for SingleGene {
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

        for chunk in state.population.chromosomes.chunks_mut(2) {
            if let [father, mother] = chunk {
                genotype.crossover_chromosome_pair_gene(father, mother, rng);
            }
        }

        if self.keep_parent {
            state.population.chromosomes.append(&mut parent_chromosomes);
        }
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
    fn require_crossover_points(&self) -> bool {
        false
    }
}

impl SingleGene {
    pub fn new(keep_parent: bool) -> Self {
        Self { keep_parent }
    }
}
