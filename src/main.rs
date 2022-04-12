use genetic_algorithm::context::Context;
use genetic_algorithm::evolve;

fn main() {
    let context = Context::new()
        .with_gene_size(100)
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20)
        .with_mutation_probability(0.1);

    if let Some(best_chromosome) = evolve::call(&context) {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }
}
