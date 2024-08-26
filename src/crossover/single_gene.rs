use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::distributions::{Distribution, Slice};
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
        let crossover_indexes = genotype.crossover_indexes();
        let crossover_index_sampler = Slice::new(&crossover_indexes).unwrap();
        let mut parent_chromosomes = if self.keep_parent {
            state.population.chromosomes.clone()
        } else {
            vec![] // throwaway to keep compiler happy
        };

        for chunk in state.population.chromosomes.chunks_mut(2) {
            if let [father, mother] = chunk {
                let index = crossover_index_sampler.sample(rng);
                std::mem::swap(&mut father.genes[*index], &mut mother.genes[*index]);
                mother.taint_fitness_score();
                father.taint_fitness_score();
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
