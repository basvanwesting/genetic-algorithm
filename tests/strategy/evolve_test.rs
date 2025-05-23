#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{
    CountOnes, CountTrue, SumDynamicMatrix, SumGenes, SumStaticMatrix,
};
use genetic_algorithm::strategy::evolve::prelude::*;

#[test]
fn build_invalid_missing_ending_condition() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
fn build_invalid_require_crossover_indexes() {
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        // .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .build();

    assert!(evolve.is_err());
    assert_eq!(
        evolve.err(),
        Some(TryFromEvolveBuilderError(
            "The provided Crossover strategy requires crossover_indexes, which the provided EvolveGenotype does not provide"
        ))
    );
}
#[test]
fn build_invalid_require_crossover_points() {
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSinglePoint::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterNoop::new())
        .build();

    assert!(evolve.is_err());
    assert_eq!(
        evolve.err(),
        Some(TryFromEvolveBuilderError(
            "The provided Crossover strategy requires crossover_points, which the provided EvolveGenotype does not provide"
        ))
    );
}

#[test]
fn call_binary_max_stale_generations_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
        evolve.best_genes().unwrap(),
        vec![true, true, true, true, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_max_stale_generations_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
        evolve.best_genes().unwrap(),
        vec![false, false, false, false, false, false, false, false, false, false]
    );
}

#[test]
fn call_binary_max_generations_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_generations(50)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
        evolve.best_genes().unwrap(),
        vec![true, true, true, true, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_max_stale_generations_and_valid_fitness_score_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_stale_generations(2)
        .with_valid_fitness_score(75)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_max_stale_generations(2)
        .with_valid_fitness_score(25)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(9)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
        evolve.best_genes().unwrap(),
        vec![true, true, true, false, true, true, true, true, true, true]
    );
}

#[test]
fn call_binary_target_fitness_score_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
        evolve.best_genes().unwrap(),
        vec![false, false, false, false, false, false, false, false, false, false]
    );
}

#[test]
fn call_binary_mass_degeneration() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(10)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(10)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_target_fitness_score(10)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
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
fn call_bit() {
    let genotype = BitGenotype::builder().with_genes_size(20).build().unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountOnes)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(20));
    assert_eq!(
        inspect::genes_to_str(&evolve.best_genes().unwrap()),
        "11111111111111111111"
    );
}

#[test]
fn call_range_f32() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new_with_precision(1e-3))
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
        evolve.best_genes().unwrap(),
        vec![0.998, 0.993, 0.979, 0.992, 0.982, 0.999, 0.987, 0.972, 0.979, 0.995],
        0.001
    ));
}

#[test]
fn call_range_usize() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
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
        evolve.best_genes().unwrap(),
        vec![9, 9, 9, 8, 9, 9, 9, 9, 9, 9]
    );
}

#[test]
fn call_range_isize() {
    let genotype = RangeGenotype::builder()
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
        .with_fitness(SumGenes::new())
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
        evolve.best_genes().unwrap(),
        vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9]
    );
}

#[test]
fn call_list() {
    let genotype = ListGenotype::builder()
        .with_genes_size(10)
        .with_allele_list((0..4).collect())
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(30));
    assert_eq!(
        evolve.best_genes().unwrap(),
        vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
    );
}

#[test]
fn call_multi_list() {
    let genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0],
            vec![0, 1, 2, 3],
        ])
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(8));
    assert_eq!(evolve.best_genes().unwrap(), vec![4, 1, 0, 3]);
}

#[test]
fn call_static_matrix() {
    let genotype = StaticMatrixGenotype::<u16, 10, 170>::builder()
        .with_genes_size(10)
        .with_allele_range(0..=10)
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumStaticMatrix::new())
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
        evolve.best_genes().unwrap(),
        Box::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    );
}

#[test]
fn call_dynamic_matrix() {
    let genotype = DynamicMatrixGenotype::<u16>::builder()
        .with_genes_size(10)
        .with_allele_range(0..=10)
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumDynamicMatrix::new())
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
        evolve.best_genes().unwrap(),
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    // after cleanup
    assert_eq!(evolve.genotype.data.len(), 0);
}

#[test]
fn call_par_fitness() {
    let genotype = ListGenotype::builder()
        .with_genes_size(10)
        .with_allele_list((0..4).collect())
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(SumGenes::new())
        .with_par_fitness(true)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        // .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    println!("{:#?}", evolve.best_genes());
    assert_eq!(evolve.best_fitness_score(), Some(30));
    assert_eq!(
        evolve.best_genes().unwrap(),
        vec![3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
    );
}

#[test]
fn population_factory_binary() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(4)
        .build()
        .unwrap();
    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(8)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_extension(ExtensionNoop::new())
        .with_reporter(StrategyReporterNoop::new())
        .with_rng_seed_from_u64(0)
        .build()
        .unwrap();

    evolve.setup(None);
    assert_eq!(
        inspect::population(&evolve.state.population),
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
