use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverSingle;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::FitnessSimpleCount;
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::mutate::MutateOnce;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::new().with_gene_size(100).build();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(1000)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(FitnessSimpleCount)
        .with_crossover(CrossoverSingle(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap()
        .call(&mut rng);

    println!("{}", evolve);
}
