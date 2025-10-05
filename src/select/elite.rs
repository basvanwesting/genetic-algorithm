use super::Select;
use crate::chromosome::Chromosome;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::prelude::*;
use std::cmp::Reverse;
use std::time::Instant;

/// Simply sort the chromosomes with fittest first. Then take the target_population_size (or full
/// population when in shortage) of the populations best and drop excess chromosomes. This approach
/// has the risk of locking in to a local optimum.
#[derive(Clone, Debug)]
pub struct Elite {
    pub replacement_rate: f32,
    pub elitism_rate: f32,
}

impl Select for Elite {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState,
        config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();

        let mut elite_chromosomes =
            self.extract_elite_chromosomes::<G>(state, config, self.elitism_rate);

        let (mut offspring, mut parents): (Vec<Chromosome>, Vec<Chromosome>) = state
            .population
            .chromosomes
            .drain(..)
            .partition(|c| c.is_offspring());

        let (new_parents_size, new_offspring_size) = self.parent_and_offspring_survival_sizes(
            parents.len(),
            offspring.len(),
            config.target_population_size - elite_chromosomes.len(),
            self.replacement_rate,
        );

        self.selection(&mut parents, new_parents_size, genotype, config, rng);
        self.selection(&mut offspring, new_offspring_size, genotype, config, rng);

        state.population.chromosomes.append(&mut elite_chromosomes);
        state.population.chromosomes.append(&mut offspring);
        state.population.chromosomes.append(&mut parents);

        self.selection(
            &mut state.population.chromosomes,
            config.target_population_size,
            genotype,
            config,
            rng,
        );

        state.add_duration(StrategyAction::Select, now.elapsed());
    }
}

impl Elite {
    pub fn new(replacement_rate: f32, elitism_rate: f32) -> Self {
        Self {
            replacement_rate,
            elitism_rate,
        }
    }

    pub fn selection<G: EvolveGenotype, R: Rng>(
        &self,
        chromosomes: &mut Vec<Chromosome>,
        selection_size: usize,
        genotype: &mut G,
        config: &EvolveConfig,
        _rng: &mut R,
    ) {
        let selection_size = std::cmp::min(selection_size, chromosomes.len());
        match config.fitness_ordering {
            FitnessOrdering::Maximize => {
                chromosomes.sort_unstable_by_key(|c| match c.fitness_score() {
                    Some(fitness_score) => Reverse(fitness_score),
                    None => Reverse(FitnessValue::MIN),
                });
            }
            FitnessOrdering::Minimize => {
                chromosomes.sort_unstable_by_key(|c| match c.fitness_score() {
                    Some(fitness_score) => fitness_score,
                    None => FitnessValue::MAX,
                });
            }
        }
        genotype.chromosome_destructor_truncate(chromosomes, selection_size);
    }
}
