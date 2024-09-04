#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverParMultiPoint};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn population_even_no_shortage() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut population: Population<BinaryGenotype> = build::population(vec![
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
        vec![false; 10],
    ]);
    population.chromosomes.reserve_exact(10);
    assert_eq!(population.chromosomes.capacity(), 14);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(1);
    CrossoverParMultiPoint::new(3, false).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    // cannot assert result as parallel execution in combination with randomness is not determinstic. Just assert it doens't panic
    assert_eq!(state.population.size(), 4);
    assert_eq!(state.population.chromosomes.capacity(), 14);
}

#[test]
fn population_even_shortage() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut population: Population<BinaryGenotype> = build::population(vec![
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
        vec![false; 10],
    ]);
    population.chromosomes.reserve_exact(10);
    assert_eq!(population.chromosomes.capacity(), 14);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 6,
        ..Default::default()
    };
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(1);
    CrossoverParMultiPoint::new(3, true).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    // cannot assert result as parallel execution in combination with randomness is not determinstic. Just assert it doens't panic
    assert_eq!(state.population.size(), 6);
    assert_eq!(state.population.chromosomes.capacity(), 14);
}
