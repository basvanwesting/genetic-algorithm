#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverParMultiPoint};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn population_even() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let population = build::population(vec![
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
        vec![false; 10],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(1);
    CrossoverParMultiPoint::new(3, false, false).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    // cannot assert result as parallel execution in combination with randomness is not determinstic. Just assert it doens't panic
    assert_eq!(state.population.size(), 4);
}

#[test]
fn population_even_keep_parents() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut population = build::population(vec![
        vec![true; 10],
        vec![false; 10],
        vec![true; 10],
        vec![false; 10],
    ]);
    population.chromosomes.reserve_exact(10);
    assert_eq!(population.chromosomes.capacity(), 14);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(1);
    CrossoverParMultiPoint::new(3, true, true).call(
        &genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    // cannot assert result as parallel execution in combination with randomness is not determinstic. Just assert it doens't panic
    assert_eq!(state.population.size(), 8);
    assert_eq!(state.population.chromosomes.capacity(), 14);
}
