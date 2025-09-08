#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::crossover::{Crossover, CrossoverClone};
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::centralized::strategy::StrategyReporterNoop;

#[test]
fn standard() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population_with_age(
        &mut genotype,
        vec![
            (vec![true, true, true], 1),
            (vec![false, false, false], 2),
            (vec![true, true, true], 1),
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 3,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        static_inspect::population_with_age(&genotype, &state.population),
        vec![
            (vec![true, true, true], 1),
            (vec![false, false, false], 2),
            (vec![true, true, true], 1),
            (vec![true, true, true], 0),
            (vec![false, false, false], 0),
        ]
    )
}
