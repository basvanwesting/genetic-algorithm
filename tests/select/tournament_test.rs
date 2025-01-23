#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::select::{Select, SelectTournament};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn maximize() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population(vec![
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

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let mut reporter = StrategyReporterNoop::<BinaryGenotype>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, &genotype, None, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        ..Default::default()
    };
    SelectTournament::new(4, 0.74).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![false, false, true],
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 8);
}

#[test]
fn minimize() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population: Population<BinaryChromosome> = build::population(vec![
        vec![false, false, false],
        vec![false, false, true],
        vec![false, true, false],
        vec![false, true, true],
        vec![true, false, false],
        vec![true, false, true],
        vec![true, true, false],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let mut reporter = StrategyReporterNoop::<BinaryGenotype>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountTrue.call_for_population(&mut state.population, &genotype, None, None);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        ..Default::default()
    };
    SelectTournament::new(4, 0.74).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![false, true, true],
        ]
    );
}

#[test]
fn fitness_ordering_with_none_fitness() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    let population: Population<BinaryChromosome> = build::population_with_fitness_scores(vec![
        (vec![false, false, false], Some(0)),
        (vec![false, false, true], None),
        (vec![false, true, false], Some(1)),
        (vec![false, true, true], Some(2)),
        (vec![true, false, false], Some(1)),
        (vec![true, false, true], Some(2)),
        (vec![true, true, false], Some(2)),
        (vec![true, true, true], Some(3)),
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let mut reporter = StrategyReporterNoop::<BinaryGenotype>::new();
    let mut rng = SmallRng::seed_from_u64(0);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        ..Default::default()
    };
    SelectTournament::new(4, 1.0).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![false, false, false], Some(0)),
            (vec![false, true, false], Some(1)),
            (vec![true, false, false], Some(1)),
            (vec![true, false, true], Some(2)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, false], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![false, false, true], None),
        ]
    );

    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        ..Default::default()
    };
    SelectTournament::new(4, 1.0).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(
        inspect::population_with_fitness_scores(&state.population),
        vec![
            (vec![false, true, true], Some(2)),
            (vec![true, false, true], Some(2)),
            (vec![true, true, false], Some(2)),
            (vec![true, false, false], Some(1)),
            (vec![true, true, true], Some(3)),
            (vec![false, true, false], Some(1)),
            (vec![false, false, true], None),
            (vec![false, false, false], Some(0)),
        ]
    );
}
