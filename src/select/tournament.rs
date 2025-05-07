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
/// target_population_size (or full population when in shortage) of the population is reached and
/// drop excess chromosomes. This approach kind of sorts the fitness first, but not very strictly.
/// This preserves a level of diversity, which avoids local optimum lock-in.
#[derive(Clone, Debug)]
pub struct Tournament {
    pub replacement_rate: f32,
    pub ageless_elitism_rate: f32,
    pub tournament_size: usize,
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

impl Tournament {
    pub fn new(replacement_rate: f32, ageless_elitism_rate: f32, tournament_size: usize) -> Self {
        Self {
            replacement_rate,
            ageless_elitism_rate,
            tournament_size,
        }
    }

    pub fn selection<G: EvolveGenotype, R: Rng>(
        &self,
        mut chromosomes: Vec<G::Chromosome>,
        selection_size: usize,
        genotype: &mut G,
        config: &EvolveConfig,
        rng: &mut R,
    ) -> Vec<G::Chromosome> {
        let mut working_population_size = chromosomes.len();
        let tournament_size = std::cmp::min(self.tournament_size, working_population_size);

        let mut selected_chromosomes: Vec<G::Chromosome> = Vec::with_capacity(selection_size);
        let mut sample_index: usize;
        let mut winning_index: usize;
        let mut sample_fitness_value: FitnessValue;
        let mut winning_fitness_value: FitnessValue;

        match config.fitness_ordering {
            FitnessOrdering::Maximize => {
                for _ in 0..selection_size {
                    winning_index = 0;
                    winning_fitness_value = FitnessValue::MIN;

                    for _ in 0..tournament_size {
                        sample_index = rng.gen_range(0..working_population_size);
                        sample_fitness_value = chromosomes[sample_index]
                            .fitness_score()
                            .unwrap_or(FitnessValue::MIN);

                        if sample_fitness_value >= winning_fitness_value {
                            winning_index = sample_index;
                            winning_fitness_value = sample_fitness_value;
                        }
                    }
                    let chromosome = chromosomes.swap_remove(winning_index);
                    selected_chromosomes.push(chromosome);
                    working_population_size -= 1;
                }
            }
            FitnessOrdering::Minimize => {
                for _ in 0..selection_size {
                    winning_index = 0;
                    winning_fitness_value = FitnessValue::MAX;

                    for _ in 0..tournament_size {
                        sample_index = rng.gen_range(0..working_population_size);
                        sample_fitness_value = chromosomes[sample_index]
                            .fitness_score()
                            .unwrap_or(FitnessValue::MAX);

                        if sample_fitness_value <= winning_fitness_value {
                            winning_index = sample_index;
                            winning_fitness_value = sample_fitness_value;
                        }
                    }
                    let chromosome = chromosomes.swap_remove(winning_index);
                    selected_chromosomes.push(chromosome);
                    working_population_size -= 1;
                }
            }
        };
        genotype.chromosome_destructor_truncate(&mut chromosomes, 0);
        selected_chromosomes
    }
}
