use genetic_algorithm::centralized::fitness::placeholders::CountStaticTrue;
use genetic_algorithm::centralized::genotype::StaticBinaryGenotype;
use genetic_algorithm::centralized::strategy::evolve::prelude::*;

fn main() {
    env_logger::init();

    let genotype = StaticBinaryGenotype::<100, 2000>::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(1000)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_fitness(CountStaticTrue::<100, 2000>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .build();

    match evolve {
        Ok(_) => unreachable!(
            "This example should not have reached this arm, we expect an invalud Evolve build."
        ),
        Err(error) => println!("Invalid Evolve build: {:?}", error),
    }
}
