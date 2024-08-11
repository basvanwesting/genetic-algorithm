use super::{Mutate, MutateEvent};
use crate::genotype::Genotype;
use crate::strategy::evolve::{EvolveConfig, EvolveReporter, EvolveState};
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

/// Selects [Chromosomes](crate::chromosome::Chromosome) in the [Population](crate::population::Population) with the dynamically
/// updated mutation_probability. Then mutates the selected chromosomes once using random mutation.
/// The mutation probability is dynamically increased or decreased to achieve a target population
/// cardinality
#[derive(Debug, Clone, Default)]
pub struct SingleGeneRandomDynamic {
    pub mutation_probability: f32,
    pub mutation_probability_step: f32,
    pub target_cardinality: usize,
}

impl Mutate for SingleGeneRandomDynamic {
    fn call<G: Genotype, R: Rng, SR: EvolveReporter<Allele = G::Allele>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G::Allele>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        if state.population.fitness_score_cardinality() < self.target_cardinality {
            self.mutation_probability =
                (self.mutation_probability + self.mutation_probability_step).min(1.0);
        } else {
            self.mutation_probability =
                (self.mutation_probability - self.mutation_probability_step).max(0.0);
        }
        reporter.on_mutate_event(
            MutateEvent::ChangeMutationProbability(format!(
                "set to {:0.3}",
                self.mutation_probability
            )),
            state,
            config,
        );

        let bool_sampler = Bernoulli::new(self.mutation_probability as f64).unwrap();
        for chromosome in state
            .population
            .chromosomes
            .iter_mut()
            .filter(|c| c.age == 0)
        {
            if bool_sampler.sample(rng) {
                genotype.mutate_chromosome_random(chromosome, rng);
            }
        }
    }
    fn report(&self) -> String {
        format!(
            "single-gene-random-dynamic: {:2.2}",
            self.mutation_probability
        )
    }
}

impl SingleGeneRandomDynamic {
    pub fn new(mutation_probability_step: f32, target_cardinality: usize) -> Self {
        Self {
            mutation_probability_step,
            target_cardinality,
            ..Default::default()
        }
    }
}
