use super::Crossover;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use itertools::Itertools;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;
use std::marker::PhantomData;
use std::time::Instant;

/// Crossover multiple gene positions from which on the rest of the genes are taken from the other
/// parent. This goes back and forth. The gene positions are chosen with uniform probability.
/// Choose between allowing duplicate crossovers on the same point or not (not much slower, as
/// crossover itself is relatively expensive).
///
/// Not allowed for [UniqueGenotype](crate::genotype::UniqueGenotype) as it would not preserve the gene uniqueness in the children.
/// Allowed for [MultiUniqueGenotype](crate::genotype::MultiUniqueGenotype) as there are valid crossover points between each new set
#[derive(Clone, Debug)]
pub struct MultiPoint<G: EvolveGenotype> {
    _phantom: PhantomData<G>,
    pub selection_rate: f32,
    pub crossover_rate: f32,
    pub crossover_sampler: Bernoulli,
    pub number_of_crossovers: usize,
    pub allow_duplicates: bool,
}
impl<G: EvolveGenotype> Crossover for MultiPoint<G> {
    type Genotype = G;

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let existing_population_size = state.population.chromosomes.len();
        let selected_population_size =
            (existing_population_size as f32 * self.selection_rate).ceil() as usize;
        state.population.increment_age();
        state
            .population
            .extend_from_within(selected_population_size);
        let iterator = state
            .population
            .chromosomes
            .iter_mut()
            .skip(existing_population_size);
        for (father, mother) in iterator.tuples() {
            if self.crossover_sampler.sample(rng) {
                genotype.crossover_chromosome_points(
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
    fn require_crossover_points(&self) -> bool {
        true
    }
}

impl<G: EvolveGenotype> MultiPoint<G> {
    /// Create a new MultiPoint crossover strategy.
    /// * `selection_rate` - fraction of parents selected for reproduction (0.5-0.8 typical)
    /// * `crossover_rate` - probability parent pair crosses over vs cloning (0.5-0.9 typical)
    /// * `number_of_crossovers` - number of crossover points along the chromosome
    /// * `allow_duplicates` - allow the same crossover point to be selected twice
    pub fn new(
        selection_rate: f32,
        crossover_rate: f32,
        number_of_crossovers: usize,
        allow_duplicates: bool,
    ) -> Self {
        let crossover_sampler = Bernoulli::new(crossover_rate as f64).unwrap();
        Self {
            _phantom: PhantomData,
            selection_rate,
            crossover_rate,
            crossover_sampler,
            number_of_crossovers,
            allow_duplicates,
        }
    }
}
