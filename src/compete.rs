use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::seq::IteratorRandom;

pub trait Compete {
    fn call<T: Gene>(&self, context: &mut Context<T>, population: Population<T>) -> Population<T>;
}

pub type TournamentSize = usize;

pub struct Tournament(pub TournamentSize);
impl Tournament {
    fn tournament_single_round<T: Gene>(
        &self,
        context: &mut Context<T>,
        population: &Population<T>,
    ) -> Option<usize> {
        let mut slice: Vec<(usize, &Chromosome<T>)> = population
            .chromosomes
            .iter()
            .enumerate()
            .choose_multiple(&mut context.rng, self.0);

        slice.sort_unstable_by_key(|a| a.1);

        if let Some(&(index, _)) = slice.last() {
            Some(index)
        } else {
            None
        }
    }
}

impl Compete for Tournament {
    fn call<T: Gene>(
        &self,
        context: &mut Context<T>,
        mut population: Population<T>,
    ) -> Population<T> {
        let mut target_chromosomes: Vec<Chromosome<T>> =
            Vec::with_capacity(context.population_size);

        for _ in 0..context.population_size {
            if let Some(winning_index) = self.tournament_single_round(context, &population) {
                let chromosome = population.chromosomes.swap_remove(winning_index);
                target_chromosomes.push(chromosome);
            } else {
                break;
            }
        }

        Population::new(target_chromosomes)
    }
}
