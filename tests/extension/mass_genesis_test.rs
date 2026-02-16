#[cfg(test)]
use crate::support::*;
use genetic_algorithm::extension::{Extension, ExtensionMassGenesis};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn removes_lesser() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .with_genes_hashing(true)
        .build()
        .unwrap();

    let mut population: Population<bool> = build::population_with_fitness_scores(vec![
        (vec![true, true, true], Some(0)),
        (vec![true, true, false], Some(1)),
        (vec![true, false, false], Some(2)),
        (vec![true, true, true], Some(0)),
        (vec![true, true, false], Some(1)),
        (vec![true, false, false], Some(2)),
        (vec![true, true, true], Some(0)),
        (vec![true, true, false], Some(1)),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    assert_eq!(population.genes_cardinality(), Some(3));
    state.population_cardinality = population.genes_cardinality();
    state.population = population;

    let config = EvolveConfig {
        target_population_size: 0,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassGenesis::new(3).after_selection_complete(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, false, false], Some(2)),
            (vec![true, true, false], Some(1)),
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn removes_lesser_no_fitness() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .with_genes_hashing(true)
        .build()
        .unwrap();

    let mut population: Population<bool> = build::population_with_fitness_scores(vec![
        (vec![true, true, true], None),
        (vec![true, true, false], None),
        (vec![true, false, false], None),
        (vec![true, true, true], None),
        (vec![true, true, false], None),
        (vec![true, false, false], None),
        (vec![true, true, true], None),
        (vec![true, true, false], None),
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    assert_eq!(population.genes_cardinality(), Some(3));
    state.population_cardinality = population.genes_cardinality();
    state.population = population;

    let config = EvolveConfig {
        target_population_size: 0,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassGenesis::new(3).after_selection_complete(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, true, true], None),
            (vec![true, true, false], None)
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}
