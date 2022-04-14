use genetic_algorithm::context::Context;
use genetic_algorithm::evolve;
use genetic_algorithm::fitness;

fn main() {
    let mut context = Context::new()
        .with_gene_size(100)
        .with_gene_values(vec![true, false])
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20)
        .with_mutation_probability(0.1)
        .with_fitness_function(fitness::count_true_values);

    println!("{}", context);

    if let Some(best_chromosome) = evolve::call(&mut context) {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }

    let mut context = Context::new()
        .with_gene_size(100)
        .with_gene_values(vec![1, 2, 3, 4])
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20)
        .with_mutation_probability(0.1)
        .with_fitness_function(fitness::sum_values);

    println!("{}", context);

    if let Some(best_chromosome) = evolve::call(&mut context) {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }
}
