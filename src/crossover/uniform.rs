use super::Crossover;
use crate::chromosome::Chromosome;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use itertools::Itertools;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::time::Instant;

/// Crossover with 50% probability for each gene to come from one of the two parents.
/// Actually implemented as `CrossoverMultiGene::new(.., <genes_size> / 2, allow_duplicates=true)`
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct Uniform {
    pub selection_rate: f32,
    pub crossover_rate: f32,
    pub crossover_sampler: Bernoulli,
}

impl Crossover for Uniform {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let number_of_crossovers = genotype.genes_size() / 2;
        let existing_population_size = state.population.chromosomes.len();
        let selected_population_size =
            (state.population.size() as f32 * self.selection_rate).ceil() as usize;
        genotype
            .chromosome_cloner_expand(&mut state.population.chromosomes, selected_population_size);
        let iterator = state
            .population
            .chromosomes
            .iter_mut()
            .skip(existing_population_size);
        for (father, mother) in iterator.tuples() {
            if self.crossover_sampler.sample(rng) {
                genotype.crossover_chromosome_genes(
                    number_of_crossovers,
                    true,
                    father,
                    mother,
                    rng,
                );
            } else {
                father.reset_age();
                mother.reset_age();
            }
        }
        if selected_population_size % 2 == 1 {
            if let Some(chromosome) = state.population.chromosomes.last_mut() {
                chromosome.reset_age();
            }
        }

        state.add_duration(StrategyAction::Crossover, now.elapsed());
    }
    fn require_crossover_indexes(&self) -> bool {
        true
    }
}

impl Uniform {
    pub fn new(selection_rate: f32, crossover_rate: f32) -> Self {
        let crossover_sampler = Bernoulli::new(crossover_rate as f64).unwrap();
        Self {
            selection_rate,
            crossover_rate,
            crossover_sampler,
        }
    }
}
