use super::Select;
use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use crate::fitness::FitnessValue;
use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::{StrategyAction, StrategyReporter, StrategyState};
use rand::prelude::*;
use std::time::Instant;

/// Run tournaments with randomly chosen chromosomes and pick a single winner. Do this untill the
/// selection_rate of the population is reached and drop excess chromosomes. This approach kind of
/// sorts the fitness first, but not very strictly. This preserves a level of diversity, which
/// avoids local optimum lock-in.
#[derive(Clone, Debug)]
pub struct Tournament {
    pub tournament_size: usize,
    pub selection_rate: f32,
}

impl Select for Tournament {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let mut working_population_size = state.population.size();
        let tournament_size = std::cmp::min(self.tournament_size, working_population_size);

        let selected_population_size = ((working_population_size as f32 * self.selection_rate)
            .ceil() as usize)
            .min(working_population_size)
            .max(2);

        let mut selected_chromosomes: Vec<G::Chromosome> =
            Vec::with_capacity(selected_population_size);
        let mut sample_index: usize;
        let mut winning_index: usize;
        let mut sample_fitness_value: FitnessValue;
        let mut winning_fitness_value: FitnessValue;

        match config.fitness_ordering {
            FitnessOrdering::Maximize => {
                for _ in 0..selected_population_size {
                    winning_index = 0;
                    winning_fitness_value = FitnessValue::MIN;

                    for _ in 0..tournament_size {
                        sample_index = rng.gen_range(0..working_population_size);
                        sample_fitness_value = state.population.chromosomes[sample_index]
                            .fitness_score()
                            .unwrap_or(FitnessValue::MIN);

                        if sample_fitness_value >= winning_fitness_value {
                            winning_index = sample_index;
                            winning_fitness_value = sample_fitness_value;
                        }
                    }
                    let chromosome = state.population.chromosomes.swap_remove(winning_index);
                    selected_chromosomes.push(chromosome);
                    working_population_size -= 1;
                }
            }
            FitnessOrdering::Minimize => {
                for _ in 0..selected_population_size {
                    winning_index = 0;
                    winning_fitness_value = FitnessValue::MAX;

                    for _ in 0..tournament_size {
                        sample_index = rng.gen_range(0..working_population_size);
                        sample_fitness_value = state.population.chromosomes[sample_index]
                            .fitness_score()
                            .unwrap_or(FitnessValue::MAX);

                        if sample_fitness_value <= winning_fitness_value {
                            winning_index = sample_index;
                            winning_fitness_value = sample_fitness_value;
                        }
                    }
                    let chromosome = state.population.chromosomes.swap_remove(winning_index);
                    selected_chromosomes.push(chromosome);
                    working_population_size -= 1;
                }
            }
        };
        genotype.chromosome_destructor_truncate(&mut state.population.chromosomes, 0);
        state
            .population
            .chromosomes
            .append(&mut selected_chromosomes);
        state.add_duration(StrategyAction::Select, now.elapsed());
    }
}

impl Tournament {
    pub fn new(tournament_size: usize, selection_rate: f32) -> Self {
        Self {
            tournament_size,
            selection_rate,
        }
    }
}
