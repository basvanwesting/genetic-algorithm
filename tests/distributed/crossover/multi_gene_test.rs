#[cfg(test)]
use crate::support::*;
use genetic_algorithm::distributed::crossover::{Crossover, CrossoverMultiGene};
use genetic_algorithm::distributed::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::distributed::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::distributed::strategy::StrategyReporterNoop;

#[test]
fn standard() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let population: Population<VecChromosome<bool>> = build::population_with_age(vec![
        (vec![true; 10], 1),
        (vec![false; 10], 1),
        (vec![true; 10], 1),
        (vec![false; 10], 1),
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 5,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverMultiGene::new(0.5, 1.0, 4, true).call(
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
                vec![true, true, true, true, false, true, true, true, true, false],
                0
            ),
            (
                vec![false, false, false, false, true, false, false, false, false, true],
                0
            ),
        ]
    )
}
