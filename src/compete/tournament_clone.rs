use super::{Compete, CompeteDispatch, Competes};
use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use crate::fitness::FitnessValue;
use crate::genotype::Genotype;
use crate::population::Population;
use crate::strategy::evolve::EvolveConfig;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;

/// Run tournaments with randomly chosen chromosomes and pick a single winner. Do this
/// target_population_size times until the required population level is reached. This approach kind
/// of sorts the fitness first, but not very strictly. This preserves a level of diversity, which
/// avoids local optimum lock-in.
///
/// Excess chromosomes, beyond the target_population_size, are dropped.
#[derive(Clone, Debug)]
pub struct TournamentClone {
    pub tournament_size: usize,
}

impl Compete for TournamentClone {
    fn call<T: Genotype, R: Rng>(
        &self,
        population: &mut Population<T>,
        evolve_config: &EvolveConfig,
        rng: &mut R,
    ) {
        let working_population_size = population.size();
        let tournament_size = std::cmp::min(self.tournament_size, working_population_size);
        let target_population_size = std::cmp::min(
            evolve_config.target_population_size,
            working_population_size,
        );

        let mut target_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(target_population_size);
        let index_sampler = Uniform::from(0..working_population_size);
        let mut winning_fitness_score: FitnessValue;
        let mut winning_index: usize;
        let mut sample_index: usize;

        for _ in 0..target_population_size {
            winning_index = 0;
            match evolve_config.fitness_ordering {
                FitnessOrdering::Maximize => winning_fitness_score = FitnessValue::MIN,
                FitnessOrdering::Minimize => winning_fitness_score = FitnessValue::MAX,
            };

            for _ in 0..tournament_size {
                sample_index = index_sampler.sample(rng);
                match evolve_config.fitness_ordering {
                    FitnessOrdering::Maximize => {
                        if population.chromosomes[sample_index]
                            .fitness_score
                            .unwrap_or(FitnessValue::MIN)
                            >= winning_fitness_score
                        {
                            winning_index = sample_index;
                        }
                    }
                    FitnessOrdering::Minimize => {
                        if population.chromosomes[sample_index]
                            .fitness_score
                            .unwrap_or(FitnessValue::MAX)
                            <= winning_fitness_score
                        {
                            winning_index = sample_index;
                        }
                    }
                }
            }
            target_chromosomes.push(population.chromosomes[winning_index].clone());
        }

        population.chromosomes = target_chromosomes;
    }
}

impl TournamentClone {
    pub fn new(tournament_size: usize) -> Self {
        Self { tournament_size }
    }
    pub fn new_dispatch(tournament_size: usize) -> CompeteDispatch {
        CompeteDispatch {
            compete: Competes::TournamentClone,
            tournament_size,
            ..Default::default()
        }
    }
}
