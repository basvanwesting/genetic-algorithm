use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::prelude::*;

pub trait Compete: Clone + std::fmt::Debug {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: Population<T>) -> Population<T>;
}

pub type TournamentSize = usize;

#[derive(Clone, Debug)]
pub struct Elite;
impl Compete for Elite {
    fn call<T: Gene>(
        &self,
        context: &mut Context<T>,
        mut population: Population<T>,
    ) -> Population<T> {
        if population.size() <= context.population_size {
            return population;
        }

        let to_drain_from_first = population.size() - context.population_size;
        if to_drain_from_first > 0 {
            population.sort();
            population.chromosomes.drain(..to_drain_from_first);
        }
        population
    }
}

#[derive(Clone, Debug)]
pub struct Tournament(pub TournamentSize);
impl Compete for Tournament {
    fn call<T: Gene>(
        &self,
        context: &mut Context<T>,
        mut population: Population<T>,
    ) -> Population<T> {
        if population.size() <= context.population_size {
            return population;
        }

        let tournament_size = self.0;
        let mut working_population_size = population.size();
        let mut target_chromosomes: Vec<Chromosome<T>> =
            Vec::with_capacity(context.population_size);
        let mut tournament_chromosomes: Vec<(usize, Option<usize>)> =
            Vec::with_capacity(tournament_size);

        for _ in 0..context.population_size {
            for _ in 0..tournament_size {
                let sample_index = context.rng.gen_range(0..working_population_size);
                tournament_chromosomes.push((
                    sample_index,
                    population.chromosomes[sample_index].fitness_score,
                ));
            }

            tournament_chromosomes.sort_unstable_by_key(|a| a.1);
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
