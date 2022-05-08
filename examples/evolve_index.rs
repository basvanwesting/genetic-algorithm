use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverAll;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::FitnessSimpleSumIndexGenotype;
use genetic_algorithm::genotype::{Genotype, IndexGenotype};
use genetic_algorithm::mutate::MutateOnce;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = IndexGenotype::builder()
        .with_gene_size(100)
        .with_gene_value_size(5)
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(1000)
        .with_max_stale_generations(20)
        .with_target_fitness_score(400)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(FitnessSimpleSumIndexGenotype)
        .with_crossover(CrossoverAll(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap()
        .call(&mut rng);

    println!("{}", evolve);
}
