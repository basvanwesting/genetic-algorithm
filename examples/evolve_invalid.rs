use genetic_algorithm::compete;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness;
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::mutate;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::new().with_gene_size(100);

    println!("{}", genotype);

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(1000)
        .with_mutate(mutate::SingleGene(0.2))
        .with_fitness(fitness::SimpleSum)
        .with_crossover(crossover::Individual(true))
        .with_compete(compete::Tournament(4))
        .call();

    println!("{}", evolve);
}
