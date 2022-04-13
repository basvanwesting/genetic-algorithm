use genetic_algorithm::context::Context;
use genetic_algorithm::evolve;
use genetic_algorithm::fitness;
use genetic_algorithm::gene::Gene;

fn main() {
    let context = Context::<bool>::new()
        .with_gene_size(100)
        .with_gene_values(vec![Gene(true), Gene(false)])
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20)
        .with_mutation_probability(0.1)
        .with_fitness_function(fitness::count_true_values);

    println!("{}", context);

    if let Some(best_chromosome) = evolve::call(&context) {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }

    let context = Context::<u8>::new()
        .with_gene_size(100)
        .with_gene_values(vec![Gene(1), Gene(2), Gene(3), Gene(4)])
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20)
        .with_mutation_probability(0.1)
        .with_fitness_function(fitness::sum_values);

    println!("{}", context);

    if let Some(best_chromosome) = evolve::call(&context) {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }
}
