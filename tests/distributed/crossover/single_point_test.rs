#[cfg(test)]
use crate::support::*;
use genetic_algorithm::distributed::crossover::{Crossover, CrossoverSinglePoint};
use genetic_algorithm::distributed::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::distributed::population::Population;
use genetic_algorithm::distributed::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::distributed::strategy::StrategyReporterNoop;

#[test]
fn standard() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(6)
        .build()
        .unwrap();

    let population: Population<bool> = build::population_with_age(vec![
        (vec![true, true, true, true, true], 1),
        (vec![false, false, false, false, false], 1),
        (vec![true, true, true, true, true], 1),
        (vec![false, false, false, false, false], 1),
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverSinglePoint::new(0.5, 1.0).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_age(&state.population),
        vec![
            (vec![true, true, true, true, true], 1),
            (vec![false, false, false, false, false], 1),
            (vec![true, true, true, true, true], 1),
            (vec![false, false, false, false, false], 1),
            (vec![true, true, false, false, false], 0),
            (vec![false, false, true, true, true], 0),
        ]
    )
}
