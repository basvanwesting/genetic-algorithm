use crate::chromosome::Chromosome;
use crate::context::Context;
use crate::population::Population;
use rand::rngs::SmallRng;
use rand::seq::IteratorRandom;
use rand::SeedableRng;

pub fn tournament(context: &Context, mut population: Population) -> Population {
    let mut rng = SmallRng::from_entropy();
    let mut target_chromosomes: Vec<Chromosome> = Vec::with_capacity(context.population_size);

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
    rng: &mut SmallRng,
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
