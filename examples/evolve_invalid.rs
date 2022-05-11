use genetic_algorithm::evolve::prelude::*;
use genetic_algorithm::fitness::placeholders::CountTrue;

fn main() {
    let genotype = BinaryGenotype::builder()
        .with_gene_size(100)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(1000)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingle(true))
        .with_compete(CompeteTournament(4))
        .build();

    match evolve {
        Ok(_) => panic!(
            "This example should not have reached this arm, we expect an invalud Evolve build."
        ),
        Err(error) => println!("Invalid Evolve build: {:?}", error),
    }
}
