use super::Crossover;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use itertools::Itertools;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::time::Instant;

/// Crossover a single gene between the parents. The gene position is chosen with uniform
/// probability.
/// The population is restored towards the target_population_size by keeping the best parents
/// alive. Excess parents are dropped.
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct SingleGene {
    pub crossover_rate: f32,
    pub crossover_sampler: Bernoulli,
    pub elitism_rate: f32,
}
impl Crossover for SingleGene {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        self.prepare_population(genotype, state, config);
        let elitism_size =
            (self.elitism_rate * config.target_population_size as f32).ceil() as usize;
        let iterator = state.population.chromosomes.iter_mut().skip(elitism_size);
        for (father, mother) in iterator.tuples() {
            if self.crossover_sampler.sample(rng) {
                genotype.crossover_chromosome_genes(1, true, father, mother, rng);
            }
        }

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
}

impl SingleGene {
    pub fn new(crossover_rate: f32, elitism_rate: f32) -> Self {
        let crossover_sampler = Bernoulli::new(crossover_rate as f64).unwrap();
        Self {
            crossover_rate,
            crossover_sampler,
            elitism_rate,
        }
    }
}
