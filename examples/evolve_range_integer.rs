use genetic_algorithm::fitness::placeholders::SumGenes;
use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    env_logger::init();

    let mut rng = SmallRng::from_entropy();
    let genotype = RangeGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0..=10)
        .with_allele_mutation_range(-1..=1)
        .build()
        .unwrap();

    println!("{}", genotype);

    let now = std::time::Instant::now();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(500)
        .with_max_stale_generations(500)
        // .with_target_fitness_score(99 * 100_000)
        // .with_par_fitness(true) // 2x slower in this case
        .with_mutate(MutateSingleGene::new(0.2)) // multi-thread
        // .with_mutate(MutateMultiGene::new(1, 0.2)) // single-thread
        .with_fitness(SumGenes::new())
        // .with_crossover(CrossoverUniform::new(false)) // multi-thread
        .with_crossover(CrossoverUniform::new(true)) // single-thread
        // .with_compete(CompeteTournament::new(4)) // multi-thread
        .with_compete(CompeteElite) // single-thread
        .call(&mut rng)
        .unwrap();

    let duration = now.elapsed();

    println!("{}", evolve);
    println!("duration: {:?}", duration);
}
