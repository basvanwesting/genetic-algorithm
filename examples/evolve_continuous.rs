use genetic_algorithm::fitness::placeholders::SumContinuousGenotype;
use genetic_algorithm::strategy::evolve::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(1000)
        .with_max_stale_generations(10000)
        .with_target_fitness_score(99)
        //.with_degeneration_range(0.0001..1.0000)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(SumContinuousGenotype(1e-5))
        .with_crossover(CrossoverUniform(true))
        .with_compete(CompeteTournament(4))
        .call(&mut rng)
        .unwrap();

    println!("{}", evolve);
}
