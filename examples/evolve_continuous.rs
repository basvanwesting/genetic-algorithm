use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverAll;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::FitnessSimpleSumContinuousGenotype;
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype};
use genetic_algorithm::mutate::MutateOnce;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = ContinuousGenotype::builder()
        .with_gene_size(100)
        .with_gene_range(0.0..1.0)
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(1000)
        .with_max_stale_generations(10000)
        .with_target_fitness_score(95)
        .with_degeneration_range(0.0001..1.0000)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(FitnessSimpleSumContinuousGenotype)
        .with_crossover(CrossoverAll(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap()
        .call(&mut rng);

    println!("{}", evolve);
}
