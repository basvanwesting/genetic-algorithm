#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::fitness::placeholders::CountStaticTrue;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::select::{Select, SelectElite};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn maximize() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population(
        &mut genotype,
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ],
    );
    assert_eq!(population.chromosomes.capacity(), 8);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountStaticTrue::<3, 10>::new().call_for_population(&mut state.population, &genotype);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 6,
        ..Default::default()
    };
    SelectElite::new(0.5, 0.02).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![true, true, true],
            vec![false, true, true],
            vec![true, false, true],
            vec![true, true, false],
            vec![false, false, true],
            vec![false, true, false]
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 8);
}

#[test]
fn minimize() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population(
        &mut genotype,
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CountStaticTrue::<3, 10>::new().call_for_population(&mut state.population, &genotype);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 6,
        ..Default::default()
    };
    SelectElite::new(0.5, 0.02).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![true, false, false],
            vec![false, true, true],
            vec![true, false, true]
        ]
    );
}

#[test]
fn fitness_ordering_with_none_fitness() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population_with_fitness_scores(
        &mut genotype,
        vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 5,
        ..Default::default()
    };
    SelectElite::new(0.5, 0.02).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(
        static_inspect::population_with_fitness_scores(&genotype, &state.population),
        vec![
            (vec![true, true, true], Some(3)),
            (vec![false, true, true], Some(2)),
            (vec![false, false, true], Some(1)),
            (vec![false, false, false], Some(0)),
            (vec![true, true, false], None),
        ]
    );

    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 5,
        ..Default::default()
    };
    SelectElite::new(0.5, 0.02).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(
        static_inspect::population_with_fitness_scores(&genotype, &state.population),
        vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]
    );
}

#[test]
fn extreme_elitism_rates() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population_with_fitness_scores(
        &mut genotype,
        vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Maximize,
        target_population_size: 5,
        ..Default::default()
    };
    SelectElite::new(0.5, 0.0).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(
        static_inspect::population_with_fitness_scores(&genotype, &state.population),
        vec![
            (vec![true, true, true], Some(3)),
            (vec![false, true, true], Some(2)),
            (vec![false, false, true], Some(1)),
            (vec![false, false, false], Some(0)),
            (vec![true, true, false], None),
        ]
    );

    let config = EvolveConfig {
        fitness_ordering: FitnessOrdering::Minimize,
        target_population_size: 5,
        ..Default::default()
    };
    SelectElite::new(0.5, 1.0).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(
        static_inspect::population_with_fitness_scores(&genotype, &state.population),
        vec![
            (vec![false, false, false], Some(0)),
            (vec![false, false, true], Some(1)),
            (vec![false, true, true], Some(2)),
            (vec![true, true, true], Some(3)),
            (vec![true, true, false], None),
        ]
    );
}
