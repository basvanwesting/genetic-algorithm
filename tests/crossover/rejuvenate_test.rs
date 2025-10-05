#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::crossover::{Crossover, CrossoverRejuvenate};
use genetic_algorithm::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

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
            (vec![true, false, false], 2),
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
    CrossoverRejuvenate::new(0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        static_inspect::population_with_age(&genotype, &state.population),
        vec![
            (vec![true, true, true], 0),
            (vec![false, false, false], 0),
            (vec![true, true, true], 1),
        ]
    )
}
