#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverSingle};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};

#[test]
fn population_even() {
    let genotype = BinaryGenotype::builder().with_gene_size(5).build().unwrap();

    let population = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverSingle(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
        ]
    )
}

#[test]
fn population_odd() {
    let genotype = BinaryGenotype::builder().with_gene_size(5).build().unwrap();

    let population = build::population(vec![
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
        vec![false, false, false, false, false],
        vec![true, true, true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverSingle(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
            vec![true, true, false, true, true],
            vec![false, false, true, false, false],
        ]
    )
}

#[test]
fn population_size_one() {
    let genotype = BinaryGenotype::builder().with_gene_size(5).build().unwrap();

    let population = build::population(vec![vec![true, false, true, false, true]]);

    let mut rng = SmallRng::seed_from_u64(0);
    let population = CrossoverSingle(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(&population),
        vec![vec![true, false, true, false, true]]
    )
}
