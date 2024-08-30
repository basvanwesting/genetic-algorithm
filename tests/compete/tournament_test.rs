#[cfg(test)]
use crate::support::*;
use genetic_algorithm::compete::{Compete, CompeteTournament};
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
    assert_eq!(population.chromosomes.capacity(), 8);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournament::new(4).call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![false, true, true],
            vec![true, false, false],
        ]
    );
    // assert_eq!(state.population.chromosomes.capacity(), 8);
}

#[test]
fn maximize_population_shortage() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population(vec![vec![false, false, false], vec![false, false, true]]);
    assert_eq!(population.chromosomes.capacity(), 2);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournament::new(4).call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![false, false, true], vec![false, false, false],]
    );
    assert_eq!(state.population.chromosomes.capacity(), 2);
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
    CompeteTournament::new(4).call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, true],
            vec![true, false, false]
        ]
    );
}

#[test]
fn minimize_population_shortage() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population(vec![vec![false, false, false], vec![false, false, true]]);
    let mut state = EvolveState::new(&genotype, population);

    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournament::new(4).call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![false, false, false], vec![false, false, true],]
    );
}

#[test]
fn minimize_population_surplus_with_none_fitness() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population_with_fitness_scores(vec![
        (vec![false, false, false], Some(0)),
        (vec![false, false, true], None),
        (vec![false, true, false], Some(1)),
        (vec![false, true, true], Some(2)),
        (vec![true, false, false], Some(1)),
        (vec![true, false, true], Some(2)),
        (vec![true, true, false], Some(2)),
        (vec![true, true, true], Some(3)),
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournament::new(4).call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![false, false, false], Some(0)),
            (vec![false, true, false], Some(1)),
            (vec![true, false, false], Some(1)),
            (vec![true, false, true], Some(2)),
        ]
    );
}

#[test]
fn minimize_population_shortage_with_none_fitness() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population = build::population_with_fitness_scores(vec![
        (vec![false, false, false], None),
        (vec![false, false, true], Some(1)),
        (vec![false, true, true], Some(2)),
    ]);

    let mut state = EvolveState::new(&genotype, population);
    let mut reporter = EvolveReporterNoop::<BinaryAllele>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 4,
        ..Default::default()
    };
    CompeteTournament::new(4).call(&mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![false, false, false], None),
        ]
    );
}
