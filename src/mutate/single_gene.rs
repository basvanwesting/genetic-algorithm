use super::Mutate;
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the
/// [Population](crate::population::Population) with the provided mutation_probability. Then
/// mutates the selected chromosomes once, where the [Genotype] determines whether this is random,
/// relative or scaled.
#[derive(Debug, Clone)]
pub struct SingleGene {
    pub mutation_probability: f32,
}

impl Mutate for SingleGene {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in state
            .population
            .chromosomes
            .iter_mut()
            .filter(|c| c.age == 0)
        {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_single(chromosome, state.current_scale_index, rng);
            }
        }
    }
    fn report(&self) -> String {
        format!("single-gene-random: {:2.2}", self.mutation_probability)
    }
}

impl SingleGene {
    pub fn new(mutation_probability: f32) -> Self {
        Self {
            mutation_probability,
        }
    }
}
