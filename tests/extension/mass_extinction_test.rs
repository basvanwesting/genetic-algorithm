#[cfg(test)]
use crate::support::*;
use genetic_algorithm::extension::{Extension, ExtensionMassExtinction};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn removes_randomly() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let mut population: Population<BinaryChromosome> = build::population(vec![
        vec![false, true, true],
        vec![false, true, false],
        vec![false, false, true],
        vec![false, false, false],
        vec![true, true, true],
        vec![true, true, false],
        vec![true, false, true],
        vec![true, false, false],
    ]);
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(8);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassExtinction::new(8, 0.75).call(
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
            vec![false, true, false],
            vec![false, true, true],
            vec![true, true, false],
            vec![false, false, true],
            vec![false, false, false],
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn never_leaves_less_than_tow() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population(vec![
        vec![false, true, true],
        vec![false, true, false],
        vec![false, false, true],
        vec![false, false, false],
        vec![true, true, true],
        vec![true, true, false],
        vec![true, false, true],
        vec![true, false, false],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    state.population_cardinality = Some(8);
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassExtinction::new(8, 0.01).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        inspect::population(&state.population),
        vec![vec![true, true, true], vec![false, true, false],]
    );
}
