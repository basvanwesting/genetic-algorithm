use super::Crossover;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::distributions::{Distribution, Slice};
use rand::Rng;

/// Crossover with a single gene position from which on the rest of the genes are taken from the
/// other parent. The gene position is chosen with uniform probability.
/// Optionally keep parents around to compete with children later on.
///
/// Not allowed for unique genotypes as it would not preserve the gene uniqueness in the children.
#[derive(Clone, Debug)]
pub struct SinglePoint {
    pub keep_parent: bool,
}
impl Crossover for SinglePoint {
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

        let crossover_points = genotype.crossover_points();
        let crossover_point_sampler = Slice::new(&crossover_points).unwrap();
        let mut parent_chromosomes = if self.keep_parent {
            state.population.chromosomes.clone()
        } else {
            vec![] // throwaway to keep compiler happy
        };

        for chunk in state.population.chromosomes.chunks_mut(2) {
            if let [father, mother] = chunk {
                let index = crossover_point_sampler.sample(rng);

                let mut father_genes_split = father.genes.split_off(*index);
                let mut mother_genes_split = mother.genes.split_off(*index);
                father.genes.append(&mut mother_genes_split);
                mother.genes.append(&mut father_genes_split);

                mother.taint_fitness_score();
                father.taint_fitness_score();
            }
        }

        if self.keep_parent {
            state.population.chromosomes.append(&mut parent_chromosomes);
        }
    }
    fn require_crossover_indexes(&self) -> bool {
        false
    }
    fn require_crossover_points(&self) -> bool {
        true
    }
}

impl SinglePoint {
    pub fn new(keep_parent: bool) -> Self {
        Self { keep_parent }
    }
}
