#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
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
fn build_invalid_zero_target_population_size() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(0)
        .with_max_stale_generations(20)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .build();

    assert_eq!(
        evolve.err(),
        Some(TryFromEvolveBuilderError(
            "Evolve requires a target_population_size > 0"
        ))
    );
}

#[test]
fn call_repeatedly_invalid_builder_or_zero_repeats() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let builder = Evolve::builder()
        .with_genotype(genotype)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4));

    // missing ending condition
    let missing_ending_condition = Some(TryFromEvolveBuilderError(
        "Evolve requires at least a max_stale_generations, max_generations or target_fitness_score ending condition",
    ));
    assert_eq!(
        builder.clone().call_repeatedly(3).err(),
        missing_ending_condition
    );
    assert_eq!(
        builder.clone().call_par_repeatedly(3).err(),
        missing_ending_condition
    );

    let builder = builder.with_max_stale_generations(20);
    let zero_repeats = Some(TryFromEvolveBuilderError("max_repeats must be at least 1"));
    assert_eq!(builder.clone().call_repeatedly(0).err(), zero_repeats);
    assert_eq!(builder.clone().call_par_repeatedly(0).err(), zero_repeats);
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
fn call_with_reporter_period_zero() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let mut strategy = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_stale_generations(10)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_reporter(EvolveReporterSimple::new_with_buffer(0))
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let mut buffer: Vec<u8> = vec![];
    strategy.flush_reporter(&mut buffer);
    let output = String::from_utf8(buffer).unwrap();
    assert!(output.contains("enter - "));
    assert!(!output.contains("periodic - "));
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
        .with_mutation_type(MutationType::Range(1))
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

#[test]
fn call_abort_flag_preset_returns_immediately() {
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let abort_flag = Arc::new(AtomicBool::new(true));
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(1000)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_abort_flag(abort_flag.clone())
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    // is_aborted() short-circuits the loop before any generation runs; best comes from setup
    assert_eq!(evolve.state.current_generation, 0);
    assert!(evolve.best_fitness_score().is_some());
}

// Fitness increases with every evaluation, so later species runs score higher. Sets the abort
// flag once `abort_after` evaluations have been done.
#[derive(Clone, Debug)]
struct IncreasingWithAbort {
    counter: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    abort_after: usize,
    abort_flag: std::sync::Arc<std::sync::atomic::AtomicBool>,
}
impl Fitness for IncreasingWithAbort {
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        use std::sync::atomic::Ordering;
        let count = self.counter.fetch_add(1, Ordering::Relaxed);
        if count >= self.abort_after {
            self.abort_flag.store(true, Ordering::Relaxed);
        }
        Some(count as FitnessValue)
    }
}

#[test]
fn call_speciated_abort_flag_returns_best_species_run() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let counter = Arc::new(AtomicUsize::new(0));
    let abort_flag = Arc::new(AtomicBool::new(false));
    let builder = |abort_after: usize| {
        Evolve::builder()
            .with_genotype(genotype.clone())
            .with_target_population_size(20)
            .with_max_generations(5)
            .with_mutate(MutateSingleGene::new(0.1))
            .with_fitness(IncreasingWithAbort {
                counter: counter.clone(),
                abort_after,
                abort_flag: abort_flag.clone(),
            })
            .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
            .with_select(SelectTournament::new(0.5, 0.02, 4))
            .with_abort_flag(abort_flag.clone())
            .with_rng_seed_from_u64(0)
    };

    // the number of evaluations of a single run, which equals the first species run
    builder(usize::MAX).call().unwrap();
    let evaluations_per_run = counter.swap(0, Ordering::Relaxed);

    // abort during the second species run, which scores higher than the first
    let (best_run, other_runs) = builder(evaluations_per_run).call_speciated(3).unwrap();
    assert!(abort_flag.load(Ordering::Relaxed));
    assert_eq!(other_runs.len(), 1);
    assert_eq!(best_run.state.current_iteration, 1);
    assert!(best_run.best_fitness_score() > other_runs[0].best_fitness_score());
}

#[test]
fn call_par_speciated_abort_flag_returns_best_species_run() {
    use std::sync::atomic::{AtomicBool, AtomicUsize};
    use std::sync::Arc;

    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    let abort_flag = Arc::new(AtomicBool::new(false));
    let (best_run, other_runs) = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_generations(5)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(IncreasingWithAbort {
            counter: Arc::new(AtomicUsize::new(0)),
            abort_after: 100,
            abort_flag: abort_flag.clone(),
        })
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_abort_flag(abort_flag.clone())
        .with_rng_seed_from_u64(0)
        .call_par_speciated(4)
        .unwrap();

    assert!(other_runs
        .iter()
        .all(|run| best_run.best_fitness_score() >= run.best_fitness_score()));
}

#[test]
fn call_repeatedly_with_rng_seed_gives_distinct_deterministic_runs() {
    let run = |par: bool| {
        let genotype = RangeGenotype::builder()
            .with_genes_size(5)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        let builder = Evolve::builder()
            .with_genotype(genotype)
            .with_target_population_size(20)
            .with_max_stale_generations(20)
            .with_mutate(MutateSingleGene::new(0.2))
            .with_fitness(SumGenes::new_with_precision(1e-3))
            .with_crossover(CrossoverUniform::new(0.7, 0.8))
            .with_select(SelectTournament::new(0.5, 0.02, 4))
            .with_rng_seed_from_u64(0);
        let (best_run, other_runs) = if par {
            builder.call_par_repeatedly(4).unwrap()
        } else {
            builder.call_repeatedly(4).unwrap()
        };
        let mut runs: Vec<(usize, Vec<f32>)> = std::iter::once(best_run)
            .chain(other_runs)
            .map(|run| (run.state.current_iteration, run.best_genes().unwrap()))
            .collect();
        runs.sort_by_key(|(iteration, _)| *iteration);
        runs
    };

    let runs = run(false);
    assert_eq!(runs.len(), 4);
    // each iteration has its own rng, so the runs differ
    for i in 1..runs.len() {
        assert_ne!(runs[0].1, runs[i].1);
    }
    // but they are still deterministic, also when run in parallel
    assert_eq!(runs, run(false));
    assert_eq!(runs, run(true));
}
