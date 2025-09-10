#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::extension::{Extension, ExtensionMassDeduplication};
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::centralized::strategy::StrategyReporterNoop;

#[test]
fn standard() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .with_genes_hashing(true)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut population = static_build::population(
        &mut genotype,
        vec![
            vec![false, true, true],
            vec![false, true, true],
            vec![false, true, true],
            vec![true, false, true],
            vec![true, false, true],
            vec![true, false, true],
            vec![true, true, true],
            vec![true, true, true],
        ],
    );
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);
    population.chromosomes.iter_mut().for_each(|chromosome| {
        let genes_hash = genotype.calculate_genes_hash(chromosome);
        chromosome.reset_state(genes_hash);
    });

    let mut state = EvolveState::new(&genotype);
    state.population_cardinality = population.genes_cardinality();
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassDeduplication::new(3).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    state.population.chromosomes.sort_by_key(|c| c.genes_hash());
    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![false, true, true],
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}

#[test]
fn never_leaves_less_than_two() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .with_genes_hashing(true)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut population = static_build::population(
        &mut genotype,
        vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ],
    );
    population.chromosomes.reserve_exact(2);
    assert_eq!(population.chromosomes.capacity(), 10);
    population.chromosomes.iter_mut().for_each(|chromosome| {
        let genes_hash = genotype.calculate_genes_hash(chromosome);
        chromosome.reset_state(genes_hash);
    });

    let mut state = EvolveState::new(&genotype);
    state.population_cardinality = population.genes_cardinality();
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    ExtensionMassDeduplication::new(1).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![vec![true, true, true], vec![true, true, true]]
    );
    assert_eq!(state.population.chromosomes.capacity(), 10);
}
