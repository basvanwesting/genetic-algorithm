use genetic_algorithm::context::Context;
use genetic_algorithm::evolve;

fn main() {
    let context = Context::new()
        .with_gene_size(100)
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20);

    let best_chromosome = evolve::call(&context);
    println!("{:#?}", best_chromosome);
}
