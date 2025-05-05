#[cfg(test)]
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::strategy::prelude::*;

#[test]
fn build_invalid_missing_variant() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        // .with_variant(StrategyVariant::Evolve(EvolveVariant::Standard))
        .with_reporter(StrategyReporterSimple::new_with_buffer(100))
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new(0.8, 0.05))
        .with_select(SelectTournament::new(4, 0.4))
        .with_rng_seed_from_u64(0);

    let strategy = builder.build();
    assert!(strategy.is_err());
    assert_eq!(
        strategy.err(),
        Some(TryFromStrategyBuilderError("StrategyVariant is required"))
    );
}

#[test]
fn call_permutate() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let mut strategy = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(StrategyVariant::Permutate(PermutateVariant::Standard))
        .with_reporter(StrategyReporterSimple::new_with_buffer(100))
        .with_target_population_size(100)
        .with_target_fitness_score(5)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new(0.8, 0.05))
        .with_select(SelectTournament::new(4, 0.4))
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    let (best_genes, best_fitness_score) = strategy.best_genes_and_fitness_score().unwrap();
    assert_eq!(best_genes, vec![true; 5]);
    assert_eq!(best_fitness_score, 5);

    // only holds buffer of best iteration
    let mut buffer: Vec<u8> = vec![];
    strategy.flush_reporter(&mut buffer);
    assert_eq!(
        Some("enter - permutate, iteration: 0"),
        String::from_utf8(buffer).unwrap().lines().next()
    );

    // actually flushes
    let mut buffer: Vec<u8> = vec![];
    strategy.flush_reporter(&mut buffer);
    assert_eq!("", String::from_utf8(buffer).unwrap());
}

#[test]
fn call_speciated_evolve() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let (mut strategy, mut others) = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(StrategyVariant::Evolve(EvolveVariant::Standard))
        .with_reporter(StrategyReporterSimple::new_with_buffer(100))
        .with_target_population_size(100)
        // .with_target_fitness_score(5)
        .with_max_stale_generations(100)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new(0.8, 0.05))
        .with_select(SelectTournament::new(4, 0.4))
        .with_rng_seed_from_u64(0)
        .call_speciated(3)
        .unwrap();

    let (best_genes, best_fitness_score) = strategy.best_genes_and_fitness_score().unwrap();
    assert_eq!(best_genes, vec![true; 5]);
    assert_eq!(best_fitness_score, 5);

    // only holds buffer of best iteration
    let mut buffer: Vec<u8> = vec![];
    strategy.flush_reporter(&mut buffer);
    assert_eq!(
        Some("enter - evolve, iteration: 0, number of seed genes: 3"),
        String::from_utf8(buffer).unwrap().lines().next()
    );

    // actually flushes
    let mut buffer: Vec<u8> = vec![];
    strategy.flush_reporter(&mut buffer);
    assert_eq!("", String::from_utf8(buffer).unwrap());

    // the other buffers
    let other_first_lines: Vec<String> = others
        .iter_mut()
        .map(|s| {
            let mut other_buffer: Vec<u8> = vec![];
            s.flush_reporter(&mut other_buffer);
            String::from_utf8(other_buffer)
                .unwrap()
                .lines()
                .next()
                .unwrap()
                .to_string()
        })
        .collect();
    assert_eq!(
        vec![
            "enter - evolve, iteration: 0",
            "enter - evolve, iteration: 1",
            "enter - evolve, iteration: 2"
        ],
        other_first_lines
    );
}

#[test]
fn call_repeatedly_hill_climb_steepest_ascent() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let (mut strategy, mut others) = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_variant(StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent))
        .with_reporter(StrategyReporterSimple::new_with_buffer(100))
        .with_target_population_size(100)
        // .with_target_fitness_score(5)
        .with_max_stale_generations(100)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.1))
        .with_crossover(CrossoverSingleGene::new(0.8, 0.05))
        .with_select(SelectTournament::new(4, 0.4))
        .with_rng_seed_from_u64(0)
        .call_repeatedly(3)
        .unwrap();

    let (best_genes, best_fitness_score) = strategy.best_genes_and_fitness_score().unwrap();
    assert_eq!(best_genes, vec![true; 5]);
    assert_eq!(best_fitness_score, 5);

    // only holds buffer of best iteration
    let mut buffer: Vec<u8> = vec![];
    strategy.flush_reporter(&mut buffer);
    assert_eq!(
        Some("enter - hill_climb/steepest_ascent, iteration: 2"),
        String::from_utf8(buffer).unwrap().lines().next()
    );

    // actually flushes
    let mut buffer: Vec<u8> = vec![];
    strategy.flush_reporter(&mut buffer);
    assert_eq!("", String::from_utf8(buffer).unwrap());

    // the other buffers
    let other_first_lines: Vec<String> = others
        .iter_mut()
        .map(|s| {
            let mut other_buffer: Vec<u8> = vec![];
            s.flush_reporter(&mut other_buffer);
            String::from_utf8(other_buffer)
                .unwrap()
                .lines()
                .next()
                .unwrap()
                .to_string()
        })
        .collect();
    assert_eq!(
        vec![
            "enter - hill_climb/steepest_ascent, iteration: 0",
            "enter - hill_climb/steepest_ascent, iteration: 1",
        ],
        other_first_lines
    );
}
