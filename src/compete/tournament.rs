use super::{Compete, TournamentSize};
use crate::chromosome::Chromosome;
use crate::fitness::FitnessOrdering;
use crate::fitness::FitnessValue;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::prelude::*;
use std::cmp::Reverse;

/// Run tournaments with randomly chosen chromosomes and pick a single winner. Do this
/// target_population_size times until the required population level is reached. This approach kind
/// of sorts the fitness first, but not very strictly. This preserves a level of diversity, which
/// avoids local optimum lock-in.
///
/// Excess chromosomes, beyond the target_population_size are dropped.
#[derive(Clone, Debug)]
pub struct Tournament(pub TournamentSize);
impl Compete for Tournament {
    fn call<T: Genotype, R: Rng>(
        &self,
        mut population: Population<T>,
        fitness_ordering: FitnessOrdering,
        target_population_size: usize,
        rng: &mut R,
    ) -> Population<T> {
        let mut working_population_size = population.size();
        let tournament_size = std::cmp::min(self.0, working_population_size);
        let target_population_size = std::cmp::min(target_population_size, working_population_size);

        let mut target_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(target_population_size);
        let mut tournament_chromosomes: Vec<(usize, Option<FitnessValue>)> =
            Vec::with_capacity(tournament_size);

        for _ in 0..target_population_size {
            for _ in 0..tournament_size {
                let sample_index = rng.gen_range(0..working_population_size);
                tournament_chromosomes.push((
                    sample_index,
                    population.chromosomes[sample_index].fitness_score,
                ));
            }

            match fitness_ordering {
                FitnessOrdering::Maximize => tournament_chromosomes.sort_unstable_by_key(|a| a.1),
                FitnessOrdering::Minimize => {
                    tournament_chromosomes.sort_unstable_by_key(|a| match a.1 {
                        Some(fitness_score) => Reverse(fitness_score),
                        None => Reverse(FitnessValue::MAX),
                    })
                }
            }
            if let Some(&(winning_index, _)) = tournament_chromosomes.last() {
                let chromosome = population.chromosomes.swap_remove(winning_index);
                target_chromosomes.push(chromosome);
                working_population_size -= 1;
                tournament_chromosomes.clear();
            } else {
                break;
            }
        }
        Population::new(target_chromosomes)
    }
}
