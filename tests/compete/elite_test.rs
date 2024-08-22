#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::{Compete, CompeteElite};
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::genotype::{BinaryAllele, BinaryGenotype, Genotype};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn maximize_population_surplus() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
        vec![false, true, true],
        vec![true, false, false],
        vec![true, false, true],
        vec![true, true, false],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteElite.call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, true, true],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]
    );
}

#[test]
fn maximize_population_shortage() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteElite.call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
        ]
    );
}

#[test]
fn minimize_population_surplus() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
        vec![false, true, true],
        vec![true, false, false],
        vec![true, false, true],
        vec![true, true, false],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteElite.call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![true, false, false],
            vec![false, false, false],
        ]
    );
}

#[test]
fn minimize_population_shortage() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteElite.call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, false, true],
            vec![false, true, false],
            vec![false, false, false],
        ]
    );
}

#[test]
fn fitness_ordering_with_none_fitness() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population_with_fitness_scores(vec![
        (vec![false, false, false], Some(0)),
        (vec![false, false, true], Some(1)),
        (vec![false, true, true], Some(2)),
        (vec![true, true, true], Some(3)),
        (vec![true, true, false], None),
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 5,
        ..Default::default()
    };
    CompeteElite.call(&mut state, &config, &mut reporter, &mut rng);
    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, true, false], None),
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
        ]
    );

    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 5,
        ..Default::default()
    };
    CompeteElite.call(&mut state, &config, &mut reporter, &mut rng);
    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![true, true, false], None),
            (vec![true, true, true], Some(3)),
            (vec![false, true, true], Some(2)),
            (vec![false, false, true], Some(1)),
            (vec![false, false, false], Some(0)),
        ]
    );
}
