#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::mutate::{Mutate, MutateMultiGene};
use genetic_algorithm::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::centralized::strategy::StrategyReporterNoop;

#[test]
fn binary_genotype() {
    let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = static_build::population(
        &mut genotype,
        vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ],
    );

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(1);
    MutateMultiGene::new(2, 0.5).call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        static_inspect::population(&genotype, &state.population),
        vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![false, false, true],
            vec![true, false, false],
        ]
    );
}
