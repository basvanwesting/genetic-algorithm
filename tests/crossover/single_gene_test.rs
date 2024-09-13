#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverSingleGene};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveState};
use genetic_algorithm::strategy::StrategyReporterNoop;

#[test]
fn population_even_no_shortage() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let population: Population<BinaryChromosome> = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);
    assert_eq!(population.chromosomes.capacity(), 4);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig {
        target_population_size: 4,
        ..Default::default()
    };
    let mut reporter = StrategyReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverSingleGene::new().call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
        ]
    );
    assert_eq!(state.population.chromosomes.capacity(), 4);
}
