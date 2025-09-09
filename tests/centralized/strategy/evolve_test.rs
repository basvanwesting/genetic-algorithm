#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::fitness::placeholders::{
    CountStaticTrue, SumDynamicRange, SumStaticRange,
};
use genetic_algorithm::centralized::genotype::{
    DynamicRangeGenotype, Genotype, StaticBinaryGenotype, StaticRangeGenotype,
};
use genetic_algorithm::centralized::strategy::evolve::prelude::*;

#[test]
fn build_invalid_missing_ending_condition() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        // .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterNoop::new())
        .build();

    assert!(evolve.is_err());
    assert_eq!(
        evolve.err(),
        Some(TryFromEvolveBuilderError(
            "Evolve requires at least a max_stale_generations, max_generations or target_fitness_score ending condition"
        ))
    );
}

#[test]
fn call_binary_max_stale_generations_maximize() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(10));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![true, true, true, true, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_max_stale_generations_minimize() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64_option(Some(0))
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(0));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![false, false, false, false, false, false, false, false, false, false]
    );
}

#[test]
fn call_binary_max_generations_maximize() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_generations(50)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(10));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![true, true, true, true, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_max_stale_generations_and_valid_fitness_score_maximize() {
    let mut genotype = StaticBinaryGenotype::<100, 200>::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_stale_generations(2)
        .with_valid_fitness_score(75)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<100, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(75));
}

#[test]
fn call_binary_max_stale_generations_and_valid_fitness_score_minimize() {
    let mut genotype = StaticBinaryGenotype::<100, 200>::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(2)
        .with_valid_fitness_score(25)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<100, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(25));
}

#[test]
fn call_binary_target_fitness_score_maximize() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(9)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(9));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![true, true, true, false, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_target_fitness_score_minimize() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(0));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![false, false, false, false, false, false, false, false, false, false]
    );
}

#[test]
fn call_binary_mass_degeneration() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(10)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionMassDegeneration::new(10, 10, 0.02))
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(10));
}

#[test]
fn call_binary_mass_extinction() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(10)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionMassExtinction::new(10, 0.1, 0.02))
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(10));
}

#[test]
fn call_binary_mass_genesis() {
    let mut genotype = StaticBinaryGenotype::<10, 200>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(10)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<10, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionMassGenesis::new(10))
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(10));
}

#[test]
fn call_range_f32() {
    let genotype = StaticRangeGenotype::<f32, 10, 200>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumStaticRange::new_with_precision(1e-3))
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(9880));
    assert!(relative_chromosome_eq(
        evolve.best_genes().unwrap().to_vec(),
        vec![0.998, 0.993, 0.979, 0.992, 0.982, 0.999, 0.987, 0.972, 0.979, 0.995],
        0.001
    ));
}

#[test]
fn call_range_u32() {
    let genotype = StaticRangeGenotype::<u32, 10, 200>::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumStaticRange::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(89));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![9, 9, 9, 8, 9, 9, 9, 9, 9, 9]
    );
}

#[test]
fn call_range_i32() {
    let genotype = StaticRangeGenotype::<i32, 10, 200>::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .with_allele_mutation_range(-1..=1)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumStaticRange::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(90));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
    );
}

#[test]
fn call_static_range() {
    let genotype = StaticRangeGenotype::<u16, 10, 170>::builder()
        .with_genes_size(10)
        .with_allele_range(0..=10)
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumStaticRange::new())
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterSimple::new(1))
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(0));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn call_dynamic_range() {
    let genotype = DynamicRangeGenotype::<u16>::builder()
        .with_genes_size(10)
        .with_allele_range(0..=10)
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumDynamicRange::new())
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterSimple::new(1))
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(0));
    assert_eq!(
        evolve.best_genes().unwrap().to_vec(),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    // after cleanup
    assert_eq!(evolve.genotype.data.len(), 0);
}

#[test]
fn population_factory_binary() {
    let mut genotype = StaticBinaryGenotype::<4, 200>::builder()
        .with_genes_size(4)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(8)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountStaticTrue::<4, 200>::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .build()
        .unwrap();

    evolve.setup();
    assert_eq!(
        static_inspect::population(&evolve.genotype, &evolve.state.population),
        vec![
            vec![false, false, true, false],
            vec![true, true, true, false],
            vec![false, true, false, true],
            vec![true, false, true, false],
            vec![false, false, true, true],
            vec![true, false, false, true],
            vec![false, true, true, false],
            vec![true, false, true, false]
        ]
    )
}
