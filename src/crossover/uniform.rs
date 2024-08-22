use super::Crossover;
use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Crossover with 50% probability for each gene to come from one of the two parents.
/// Optionally keep parents around to compete with children later on.
///
/// Not allowed for unique genotypes as it would not preserve the gene uniqueness in the children.
#[derive(Clone, Debug)]
pub struct Uniform {
    pub keep_parent: bool,
}
impl Crossover for Uniform {
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
        let crossover_indexes = genotype.crossover_indexes();
        let bool_sampler = Bernoulli::new(0.5).unwrap();

        if self.keep_parent {
            let mut child_chromosomes: Vec<Chromosome<G::Allele>> =
                Vec::with_capacity(state.population.size());

            for chunk in state.population.chromosomes.chunks(2) {
                if let [father, mother] = chunk {
                    let mut child_father_genes = father.genes.clone();
                    let mut child_mother_genes = mother.genes.clone();

                    for index in &crossover_indexes {
                        if bool_sampler.sample(rng) {
                            std::mem::swap(
                                &mut child_father_genes[*index],
                                &mut child_mother_genes[*index],
                            );
                        }
                    }

                    // no need to taint_fitness_score as it is initialized with None
                    child_chromosomes.push(Chromosome::new(child_father_genes));
                    child_chromosomes.push(Chromosome::new(child_mother_genes));
                }
            }
            state.population.chromosomes.append(&mut child_chromosomes);
        } else {
            for chunk in state.population.chromosomes.chunks_mut(2) {
                if let [father, mother] = chunk {
                    for index in &crossover_indexes {
                        if bool_sampler.sample(rng) {
                            std::mem::swap(&mut father.genes[*index], &mut mother.genes[*index]);
                        }
                    }
                    mother.taint_fitness_score();
                    father.taint_fitness_score();
                }
            }
        }
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
    fn require_crossover_points(&self) -> bool {
        false
    }
}

impl Uniform {
    pub fn new(keep_parent: bool) -> Self {
        Self { keep_parent }
    }
}
