use crate::chromosome::Chromosome;
use crate::context::Context;

pub fn call(context: &Context) -> Option<Chromosome> {
    let mut population = context.permutation_population_factory();
    population.calculate_fitness(&context);
    population.sort();
    if let Some(best_chromosome) = population.chromosomes.last() {
        Some(best_chromosome.clone())
    } else {
        None
    }
}
