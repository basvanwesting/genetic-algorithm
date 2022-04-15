use genetic_algorithm::compete;
use genetic_algorithm::context::Context;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness;
use genetic_algorithm::gene::ContinuousGene;
use genetic_algorithm::mutate;

fn main() {
    //example_binary();
    //example_discrete();
    example_continuous();
}

#[allow(dead_code)]
fn example_binary() {
    let context = Context::new()
        .with_gene_size(100)
        .with_gene_values(vec![true, false])
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20);

    println!("{}", context);

    let evolve = Evolve::new(context)
        .with_mutate(mutate::SingleGene(0.2))
        .with_fitness(fitness::SimpleSum)
        .with_crossover(crossover::Individual)
        .with_compete(compete::Tournament)
        .call();
    if let Some(best_chromosome) = evolve.best_chromosome {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }
}

#[allow(dead_code)]
fn example_discrete() {
    let context = Context::new()
        .with_gene_size(100)
        .with_gene_values(vec![1, 2, 3, 4])
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20);

    println!("{}", context);

    let evolve = Evolve::new(context)
        .with_mutate(mutate::SingleGene(0.2))
        .with_fitness(fitness::SimpleSum)
        .with_crossover(crossover::Individual)
        .with_compete(compete::Tournament)
        .call();
    if let Some(best_chromosome) = evolve.best_chromosome {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }
}

#[allow(dead_code)]
fn example_continuous() {
    let context = Context::<ContinuousGene>::new()
        .with_gene_size(100)
        .with_population_size(1000)
        .with_tournament_size(4)
        .with_max_stale_generations(20);

    println!("{}", context);

    let evolve = Evolve::new(context)
        .with_mutate(mutate::SingleGene(0.2))
        .with_fitness(fitness::SimpleSum)
        .with_crossover(crossover::Individual)
        .with_compete(compete::Tournament)
        .call();
    if let Some(best_chromosome) = evolve.best_chromosome {
        println!("best chromosome: {}", best_chromosome);
    } else {
        println!("no best chromosome");
    }
}
