use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::Rng;

/// Crossover multiple genes between the parents. The gene positions are chosen with uniform
/// probability. Optionally keep parents around to compete with children later on.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct MultiGene {
    pub number_of_crossovers: usize,
    pub keep_parent: bool,
}
impl Crossover for MultiGene {
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
                genotype.crossover_chromosome_pair_multi_gene(
                    self.number_of_crossovers,
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
    fn require_crossover_points(&self) -> bool {
        false
    }
}

impl MultiGene {
    pub fn new(number_of_crossovers: usize, keep_parent: bool) -> Self {
        Self {
            number_of_crossovers,
            keep_parent,
        }
    }
}
