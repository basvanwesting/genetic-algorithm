use genetic_algorithm::evolve::prelude::*;
use genetic_algorithm::fitness::placeholders::CountTrue;

fn main() {
    let mut rng = rand::thread_rng();
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(100)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverAll(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    println!("{}", evolve);
}
