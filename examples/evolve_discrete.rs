use genetic_algorithm::compete;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness;
use genetic_algorithm::genotype::DiscreteGenotype;
use genetic_algorithm::mutate;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let rng = SmallRng::from_entropy();
    let genotype = DiscreteGenotype::new()
        .with_gene_size(100)
        .with_gene_values(vec![1, 2, 3, 4])
        .build();

    println!("{}", genotype);

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(1000)
        .with_max_stale_generations(20)
        .with_target_fitness_score(400)
        .with_mutate(mutate::SingleGene(0.2))
        .with_fitness(fitness::SimpleSum)
        .with_crossover(crossover::All(true))
        .with_compete(compete::Tournament(4))
        //.with_compete(compete::Elite)
        .call();

    println!("{}", evolve);
}
