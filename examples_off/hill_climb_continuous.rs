use genetic_algorithm::fitness::placeholders::SumContinuousGenotype;
use genetic_algorithm::strategy::hill_climb::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    env_logger::init();

    let mut rng = SmallRng::from_entropy();
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();

    println!("{}", genotype);

    let now = std::time::Instant::now();

    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        .with_variant(HillClimbVariant::Stochastic)
        .with_target_fitness_score(99 * 100_000)
        .with_scaling(Scaling::new(1.0, 0.8, 1e-5))
        .with_fitness(SumContinuousGenotype(1e-5))
        .call(&mut rng)
        .unwrap();

    let duration = now.elapsed();

    println!("{}", hill_climb);
    println!("duration: {:?}", duration);
}
