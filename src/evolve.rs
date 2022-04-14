use crate::chromosome::Chromosome;
use crate::competition;
use crate::context::Context;
use crate::crossover;
use crate::gene::Gene;
use crate::mutation;

pub fn call<T: Gene>(context: &mut Context<T>) -> Option<Chromosome<T>> {
    let mut generation = 0;
    let mut best_generation = 0;
    let mut new_population = context.random_population_factory();
    let mut best_chromosome = new_population.best_chromosome().unwrap().clone();

    while generation - best_generation < context.max_stale_generations {
        let mut parent_population = new_population;
        let mut child_population = crossover::individual(context, &parent_population);
        mutation::single_gene(context, &mut child_population);
        child_population.calculate_fitness(&context);
        child_population.merge(&mut parent_population);
        new_population = competition::tournament(context, child_population);

        generation += 1;
        println!(
            "generation {:?}, best chromosome {}",
            generation, best_chromosome
        );

        new_population.sort();
        if let Some(new_best_chromosome) = new_population.best_chromosome() {
            if new_best_chromosome > &best_chromosome {
                best_chromosome = new_best_chromosome.clone();
                best_generation = generation;
            }
        }
    }
    Some(best_chromosome)
}
