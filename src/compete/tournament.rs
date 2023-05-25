use super::Compete;
use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use crate::fitness::FitnessValue;
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::prelude::*;

/// Run tournaments with randomly chosen chromosomes and pick a single winner. Do this
/// target_population_size times until the required population level is reached. This approach kind
/// of sorts the fitness first, but not very strictly. This preserves a level of diversity, which
/// avoids local optimum lock-in.
///
/// Excess chromosomes, beyond the target_population_size, are dropped.
#[derive(Clone, Debug)]
pub struct Tournament {
    pub tournament_size: usize,
}

impl Compete for Tournament {
    fn call<T: Genotype, R: Rng>(
        &mut self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    ) {
        let mut working_population_size = population.size();
        let tournament_size = std::cmp::min(self.tournament_size, working_population_size);
        let target_population_size = std::cmp::min(
            evolve_config.target_population_size,
            working_population_size,
        );

        let mut target_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(target_population_size);
        let mut sample_index: usize;
        let mut winning_index: usize;
        let mut sample_fitness_value: FitnessValue;
        let mut winning_fitness_value: FitnessValue;

        for _ in 0..target_population_size {
            winning_index = 0;
            match evolve_config.fitness_ordering {
                FitnessOrdering::Maximize => winning_fitness_value = FitnessValue::MIN,
                FitnessOrdering::Minimize => winning_fitness_value = FitnessValue::MAX,
            };

            for _ in 0..tournament_size {
                sample_index = rng.gen_range(0..working_population_size);
                match evolve_config.fitness_ordering {
                    FitnessOrdering::Maximize => {
                        sample_fitness_value = population.chromosomes[sample_index]
                            .fitness_score
                            .unwrap_or(FitnessValue::MIN);

                        if sample_fitness_value >= winning_fitness_value {
                            winning_index = sample_index;
                            winning_fitness_value = sample_fitness_value;
                        }
                    }
                    FitnessOrdering::Minimize => {
                        sample_fitness_value = population.chromosomes[sample_index]
                            .fitness_score
                            .unwrap_or(FitnessValue::MAX);

                        if sample_fitness_value <= winning_fitness_value {
                            winning_index = sample_index;
                            winning_fitness_value = sample_fitness_value;
                        }
                    }
                }
            }

            let chromosome = population.chromosomes.swap_remove(winning_index);
            target_chromosomes.push(chromosome);
            working_population_size -= 1;
        }

        population.chromosomes = target_chromosomes;
    }
}

impl Tournament {
    pub fn new(tournament_size: usize) -> Self {
        Self { tournament_size }
    }
}
