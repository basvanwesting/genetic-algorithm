use genetic_algorithm::fitness::placeholders::SumContinuousGenotype;
use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..1.0)
        .build()
        .unwrap();

    println!("{}", genotype);

    let now = std::time::Instant::now();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(100)
        .with_max_stale_generations(100)
        .with_target_fitness_score(99 * 100_000)
        //.with_degeneration_range(0.0001..1.0000)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(SumContinuousGenotype(1e-5))
        .with_crossover(CrossoverUniform(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    let duration = now.elapsed();

    println!("{}", evolve);
    println!("duration: {:?}", duration);
}
