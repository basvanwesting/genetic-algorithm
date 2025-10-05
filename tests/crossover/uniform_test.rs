#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::crossover::{Crossover, CrossoverUniform};
use genetic_algorithm::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn standard() {
    let mut genotype = StaticBinaryGenotype::<10, 10>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population_with_age(
        &mut genotype,
        vec![
            (vec![true; 10], 1),
            (vec![false; 10], 1),
            (vec![true; 10], 1),
            (vec![false; 10], 1),
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverUniform::new(0.5, 1.0).call(
        &mut genotype,
        &mut state,
        &config,
        &mut reporter,
        &mut rng,
    );

    assert_eq!(
        static_inspect::population_with_age(&genotype, &state.population),
        vec![
            (vec![true; 10], 1),
            (vec![false; 10], 1),
            (vec![true; 10], 1),
            (vec![false; 10], 1),
            (
                vec![true, true, true, true, false, true, true, true, false, false],
                0
            ),
            (
                vec![false, false, false, false, true, false, false, false, true, true],
                0
            ),
        ]
    )
}
