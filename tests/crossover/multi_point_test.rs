#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverMultiPoint};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn standard() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population_with_age(vec![
        (vec![true; 10], 1),
        (vec![false; 10], 1),
        (vec![true; 10], 1),
        (vec![false; 10], 1),
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(1);
    CrossoverMultiPoint::new(0.5, 1.0, 3, true).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population_with_age(&state.population),
        vec![
            (vec![true; 10], 1),
            (vec![false; 10], 1),
            (vec![true; 10], 1),
            (vec![false; 10], 1),
            (
                vec![false, true, true, true, true, true, true, false, false, false],
                0
            ),
            (
                vec![true, false, false, false, false, false, false, true, true, true],
                0
            ),
        ]
    )
}
