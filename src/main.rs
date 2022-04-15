use genetic_algorithm::context::Context;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness;
use genetic_algorithm::mutate::MutateSingleGene;

fn main() {
    let context = Context::new()
        .with_gene_size(100)
        .with_gene_values(vec![true, false])
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20)
        .with_mutation_probability(0.1)
        .with_fitness_function(fitness::count_true_values);

    println!("{}", context);

    let evolve = Evolve::new(context, MutateSingleGene).call();
    if let Some(best_chromosome) = evolve.best_chromosome {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }

    let context = Context::new()
        .with_gene_size(100)
        .with_gene_values(vec![1, 2, 3, 4])
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20)
        .with_mutation_probability(0.1)
        .with_fitness_function(fitness::sum_discrete_values);

    println!("{}", context);

    let evolve = Evolve::new(context, MutateSingleGene).call();
    if let Some(best_chromosome) = evolve.best_chromosome {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }

    let context = Context::new()
        .with_gene_size(100)
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20)
        .with_mutation_probability(0.1)
        .with_fitness_function(fitness::sum_continuous_values);

    println!("{}", context);

    let evolve = Evolve::new(context, MutateSingleGene).call();
    if let Some(best_chromosome) = evolve.best_chromosome {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }
}
