use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::meta::prelude::*;

fn main() {
    env_logger::init();

    let rounds = 10;
    let target_population_sizes = vec![1, 2, 3, 4, 5, 10];
    let max_stale_generations_options = vec![Some(100)];
    let target_fitness_score_options = vec![Some(0)];
    let mutates = vec![
        MutateOnce::new(0.05).into(),
        MutateOnce::new(0.1).into(),
        MutateOnce::new(0.2).into(),
        MutateOnce::new(0.3).into(),
        MutateOnce::new(0.4).into(),
        MutateOnce::new(0.5).into(),
    ];
    let crossovers = vec![
        CrossoverClone::new(false).into(),
        CrossoverClone::new(true).into(),
        CrossoverSingleGene::new(false).into(),
        CrossoverSingleGene::new(true).into(),
        CrossoverSinglePoint::new(false).into(),
        CrossoverSinglePoint::new(true).into(),
        CrossoverUniform::new(false).into(),
        CrossoverUniform::new(true).into(),
    ];
    let competes = vec![
        CompeteElite::new().into(),
        CompeteTournament::new(2).into(),
        CompeteTournament::new(4).into(),
        CompeteTournament::new(8).into(),
    ];
    let extensions = vec![
        ExtensionNoop::new().into(),
        ExtensionMassDegeneration::new(0.9, 10).into(),
        ExtensionMassExtinction::new(0.9, 0.1).into(),
        ExtensionMassGenesis::new(0.9).into(),
        ExtensionMassInvasion::new(0.9, 0.1).into(),
    ];
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let fitness = CountTrue;
    let evolve_builder = EvolveBuilder::new()
        .with_genotype(genotype)
        .with_fitness(fitness)
        .with_fitness_ordering(FitnessOrdering::Minimize);
    let evolve_fitness_to_micro_second_factor = 1_000_000;

    let config = MetaConfig::builder()
        .with_evolve_builder(evolve_builder)
        .with_evolve_fitness_to_micro_second_factor(evolve_fitness_to_micro_second_factor)
        .with_rounds(rounds)
        .with_target_population_sizes(target_population_sizes)
        .with_max_stale_generations_options(max_stale_generations_options)
        .with_target_fitness_score_options(target_fitness_score_options)
        .with_mutates(mutates)
        .with_crossovers(crossovers)
        .with_competes(competes)
        .with_extensions(extensions)
        .build()
        .unwrap();

    let permutate = MetaPermutate::new(&config).call();
    println!();
    println!("{}", permutate);
}
