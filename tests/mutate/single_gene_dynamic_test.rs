#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::mutate::{Mutate, MutateSingleGeneDynamic};
use genetic_algorithm::population::Population;
use genetic_algorithm::strategy::evolve::{EvolveConfig, EvolveReporterNoop, EvolveState};

#[test]
fn binary_genotype() {
    let mut genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population: Population<BinaryGenotype> = build::population(vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);

    let mut state = EvolveState::new(&genotype);
    state.population = population;
    let config = EvolveConfig::new();
    let mut reporter = EvolveReporterNoop::new();
    let mut rng = SmallRng::seed_from_u64(0);
    let mut mutate = MutateSingleGeneDynamic::new(0.1, 2);
    let mut fitness = CountTrue;
    assert_eq!(mutate.mutation_probability, 0.0);
    fitness.call_for_population(&mut state.population, &mut genotype, None);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.1);
    fitness.call_for_population(&mut state.population, &mut genotype, None);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.2);
    fitness.call_for_population(&mut state.population, &mut genotype, None);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.1);
    fitness.call_for_population(&mut state.population, &mut genotype, None);
    mutate.call(&mut genotype, &mut state, &config, &mut reporter, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.0);

    assert_eq!(
        inspect::population(&state.population),
        vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, false, true],
        ]
    );
}
