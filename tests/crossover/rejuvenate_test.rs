#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverRejuvenate};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn standard() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<bool> = build::population_with_age(vec![
        (vec![true, true, true], 0),
        (vec![false, false, false], 1),
        (vec![true, false, false], 1),
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 3,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    state.population.increment_age();
    CrossoverRejuvenate::new(0.5).call(&genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population_with_age(&state.population),
        vec![
            (vec![true, true, true], 0),
            (vec![false, false, false], 0),
            (vec![true, true, true], 1),
        ]
    )
}

#[test]
fn low_and_high_selection_rate_without_chromosome_recycling() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .with_chromosome_recycling(false)
        .build()
        .unwrap();
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);

    for selection_rate in [0.25, 1.5] {
        let chromosomes = vec![
            build::chromosome(vec![true, true, true]),
            build::chromosome(vec![false, false, false]),
            build::chromosome(vec![true, false, false]),
            build::chromosome(vec![false, true, false]),
        ];
        let mut state = EvolveState::new(&genotype);
        state.population = Population::new(chromosomes, false);
        CrossoverRejuvenate::new(selection_rate).call(
            &genotype,
            &mut state,
            &config,
            &mut reporter,
            &mut rng,
        );
        assert_eq!(state.population.size(), 4);
    }
}
