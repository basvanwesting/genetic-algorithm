use super::Crossover;
use crate::centralized::genotype::EvolveGenotype;
use crate::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use crate::centralized::strategy::{StrategyAction, StrategyReporter, StrategyState};
use itertools::Itertools;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::time::Instant;

/// Crossover multiple genes between the parents. The gene positions are chosen with uniform
/// probability.
/// Choose between allowing duplicate crossovers of the same gene or not (~2x slower).
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) and
/// [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as it would not preserve the gene
/// uniqueness in the children.
#[derive(Clone, Debug)]
pub struct MultiGene {
    pub selection_rate: f32,
    pub crossover_rate: f32,
    pub crossover_sampler: Bernoulli,
    pub number_of_crossovers: usize,
    pub allow_duplicates: bool,
}
impl Crossover for MultiGene {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
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
                    self.number_of_crossovers,
                    self.allow_duplicates,
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

impl MultiGene {
    pub fn new(
        selection_rate: f32,
        crossover_rate: f32,
        number_of_crossovers: usize,
        allow_duplicates: bool,
    ) -> Self {
        let crossover_sampler = Bernoulli::new(crossover_rate as f64).unwrap();
        Self {
            selection_rate,
            crossover_rate,
            crossover_sampler,
            number_of_crossovers,
            allow_duplicates,
        }
    }
}
