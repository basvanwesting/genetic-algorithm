#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::mutate::{Mutate, MutateSingleGeneRandomDynamic};

#[test]
fn binary_genotype() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population = &mut build::population(vec![
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

    let mut rng = SmallRng::seed_from_u64(0);
    let mut mutate = MutateSingleGeneRandomDynamic::new(0.1, 0.9);
    let mut fitness = CountTrue;
    assert_eq!(mutate.mutation_probability, 0.0);
    fitness.call_for_population_single_thread(population);
    mutate.call(&genotype, population, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.1);
    fitness.call_for_population_single_thread(population);
    mutate.call(&genotype, population, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.2);
    fitness.call_for_population_single_thread(population);
    mutate.call(&genotype, population, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.1);
    fitness.call_for_population_single_thread(population);
    mutate.call(&genotype, population, &mut rng);
    assert_eq!(mutate.mutation_probability, 0.0);

    assert_eq!(
        inspect::population(population),
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
