use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::meta::prelude::*;

fn main() {
    let rounds = 10;
    let population_sizes = vec![1, 2, 3, 4, 5, 10];
    let max_stale_generations_options = vec![Some(100)];
    let target_fitness_score_options = vec![Some(0)];
    let degeneration_range_options = vec![None, Some(0.001..0.995)];
    let mutates = vec![
        MutateDispatch(Mutates::Once, 0.05),
        MutateDispatch(Mutates::Once, 0.1),
        MutateDispatch(Mutates::Once, 0.2),
        MutateDispatch(Mutates::Once, 0.3),
        MutateDispatch(Mutates::Once, 0.4),
        MutateDispatch(Mutates::Once, 0.5),
    ];
    let crossovers = vec![
        CrossoverDispatch(Crossovers::Single, true),
        CrossoverDispatch(Crossovers::Single, false),
        CrossoverDispatch(Crossovers::All, true),
        CrossoverDispatch(Crossovers::All, false),
        CrossoverDispatch(Crossovers::Range, true),
        CrossoverDispatch(Crossovers::Range, false),
        CrossoverDispatch(Crossovers::Clone, true),
        CrossoverDispatch(Crossovers::Clone, false),
    ];
    let competes = vec![
        CompeteDispatch(Competes::Elite, 0),
        CompeteDispatch(Competes::Tournament, 2),
        CompeteDispatch(Competes::Tournament, 4),
        CompeteDispatch(Competes::Tournament, 8),
    ];
    let genotype = BinaryGenotype::builder()
        .with_gene_size(10)
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
        .with_population_sizes(population_sizes)
        .with_max_stale_generations_options(max_stale_generations_options)
        .with_target_fitness_score_options(target_fitness_score_options)
        .with_degeneration_range_options(degeneration_range_options)
        .with_mutates(mutates)
        .with_crossovers(crossovers)
        .with_competes(competes)
        .build()
        .unwrap();

    let permutate = MetaPermutate::new(&config).call();
    println!();
    println!("{}", permutate);
}
