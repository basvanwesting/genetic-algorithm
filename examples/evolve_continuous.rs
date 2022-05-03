use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverAll;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness;
use genetic_algorithm::genotype::ContinuousGenotype;
use genetic_algorithm::mutate::MutateSingleGene;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let rng = SmallRng::from_entropy();
    let genotype = ContinuousGenotype::new().with_gene_size(100).build();

    println!("{}", genotype);

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(1000)
        .with_max_stale_generations(10000)
        .with_target_fitness_score(95)
        .with_degeneration_range(0.0001..1.0000)
        .with_mutate(MutateSingleGene(0.2))
        .with_fitness(fitness::SimpleSumContinuousGenotype)
        .with_crossover(CrossoverAll(true))
        .with_compete(CompeteTournament(4))
        .call();

    println!("{}", evolve);
}
