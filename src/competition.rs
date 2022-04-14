use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::seq::IteratorRandom;

pub fn tournament<T: Gene>(
    context: &mut Context<T>,
    mut population: Population<T>,
) -> Population<T> {
    let mut target_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(context.population_size);

    for _ in 0..context.population_size {
        if let Some(winning_index) = tournament_single_round(context, &population) {
            let chromosome = population.chromosomes.swap_remove(winning_index);
            target_chromosomes.push(chromosome);
        } else {
            break;
        }
    }

    Population::new(target_chromosomes)
}

fn tournament_single_round<T: Gene>(
    context: &mut Context<T>,
    population: &Population<T>,
) -> Option<usize> {
    let mut slice: Vec<(usize, &Chromosome<T>)> = population
        .chromosomes
        .iter()
        .enumerate()
        .choose_multiple(&mut context.rng, context.tournament_size);

    slice.sort_unstable_by_key(|a| a.1);

    if let Some(&(index, _)) = slice.last() {
        Some(index)
    } else {
        None
    }
}
