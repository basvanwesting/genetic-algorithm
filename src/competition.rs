use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::fitness;
use crate::population::Population;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::thread_rng;

pub fn tournament(context: &Context, mut population: Population) -> Population {
    let mut rng = thread_rng();

    population
        .chromosomes
        .iter_mut()
        .for_each(|o| o.fitness = Some(fitness::simple_sum(o)));

    let mut target_chromosomes: Vec<Chromosome> = vec![];

    for _ in 0..context.population_size {
        if let Some(winning_index) =
            tournament_single_round(&population, context.tournament_size, &mut rng)
        {
            let chromosome = population.chromosomes.swap_remove(winning_index);
            target_chromosomes.push(chromosome);
        } else {
            break;
        }
    }
    Population::new(target_chromosomes)
}

fn tournament_single_round(
    population: &Population,
    size: usize,
    rng: &mut ThreadRng,
) -> Option<usize> {
    let mut slice: Vec<(usize, &Chromosome)> = population
        .chromosomes
        .iter()
        .enumerate()
        .choose_multiple(rng, size);

    slice.sort_unstable_by_key(|a| a.1);

    if let Some(&(index, _)) = slice.last() {
        Some(index)
    } else {
        None
    }
}
