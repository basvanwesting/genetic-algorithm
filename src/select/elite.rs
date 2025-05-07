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
    pub ageless_elitism_rate: f32,
    pub max_age: Option<usize>,
}

impl Select for Elite {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();

        let mut ageless_elite_chromosomes =
            self.extract_ageless_elite_chromosomes(state, config, self.ageless_elitism_rate);

        let (offspring, parents): (Vec<G::Chromosome>, Vec<G::Chromosome>) = state
            .population
            .chromosomes
            .drain(..)
            .partition(|c| c.age() == 0);

        let (new_parents_size, new_offspring_size) = self.survival_sizes(
            parents.len(),
            offspring.len(),
            config.target_population_size - ageless_elite_chromosomes.len(),
            self.replacement_rate,
        );

        let mut parents = self.selection(parents, new_parents_size, genotype, config, rng);
        let mut offspring = self.selection(offspring, new_offspring_size, genotype, config, rng);

        state
            .population
            .chromosomes
            .append(&mut ageless_elite_chromosomes);
        state.population.chromosomes.append(&mut offspring);
        state.population.chromosomes.append(&mut parents);

        state.add_duration(StrategyAction::Select, now.elapsed());
    }
}

impl Elite {
    pub fn new(replacement_rate: f32, ageless_elitism_rate: f32, max_age: Option<usize>) -> Self {
        Self {
            replacement_rate,
            ageless_elitism_rate,
            max_age,
        }
    }

    pub fn selection<G: EvolveGenotype, R: Rng>(
        &self,
        mut chromosomes: Vec<G::Chromosome>,
        selection_size: usize,
        genotype: &mut G,
        config: &EvolveConfig,
        _rng: &mut R,
    ) -> Vec<G::Chromosome> {
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
        genotype.chromosome_destructor_truncate(&mut chromosomes, selection_size);
        chromosomes
    }
}
