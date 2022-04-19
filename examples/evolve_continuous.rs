use genetic_algorithm::compete;
use genetic_algorithm::context::Context;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness;
use genetic_algorithm::gene::ContinuousGene;
use genetic_algorithm::mutate;

fn main() {
    let context = Context::<ContinuousGene>::new()
        .with_gene_size(100)
        .with_population_size(1000);

    println!("{}", context);

    let evolve = Evolve::new(context)
        .with_max_stale_generations(10000)
        .with_target_fitness_score(95)
        .with_degeneration_range(0.0001..1.0000)
        .with_mutate(mutate::SingleGene(0.2))
        .with_fitness(fitness::SimpleSum)
        .with_crossover(crossover::Individual)
        .with_compete(compete::Tournament(4))
        //.with_compete(compete::Elite)
        .call();

    println!("{}", evolve);
}
