use super::Crossover;
use crate::chromosome::Chromosome;
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
    fn call<G: Genotype, R: Rng + Clone + Send + Sync, SR: EvolveReporter<Allele = G::Allele>>(
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
        if self.keep_parent {
            let mut child_chromosomes: Vec<Chromosome<G::Allele>> =
                Vec::with_capacity(state.population.size());

            for chunk in state.population.chromosomes.chunks(2) {
                if let [father, mother] = chunk {
                    let index = crossover_point_sampler.sample(rng);
                    let mut child_father_genes = father.genes.clone();
                    let mut child_mother_genes = mother.genes.clone();

                    let mut child_father_genes_split = child_father_genes.split_off(*index);
                    let mut child_mother_genes_split = child_mother_genes.split_off(*index);
                    child_father_genes.append(&mut child_mother_genes_split);
                    child_mother_genes.append(&mut child_father_genes_split);

                    // no need to taint_fitness_score as it is initialized with None
                    child_chromosomes.push(Chromosome::new(child_father_genes));
                    child_chromosomes.push(Chromosome::new(child_mother_genes));
                }
            }
            state.population.chromosomes.append(&mut child_chromosomes);
        } else {
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
