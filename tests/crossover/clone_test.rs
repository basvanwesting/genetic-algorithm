#[cfg(test)]
use crate::support::*;
use genetic_algorithm::crossover::{Crossover, CrossoverClone};
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};

#[test]
fn population_odd() {
    let genotype = BinaryGenotype::builder().with_gene_size(3).build().unwrap();

    let population = &mut build::population(vec![
        vec![true, true, true],
        vec![false, false, false],
        vec![true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
        ]
    )
}

#[test]
fn population_odd_keep_parents() {
    let genotype = BinaryGenotype::builder().with_gene_size(3).build().unwrap();

    let population = &mut build::population(vec![
        vec![true, true, true],
        vec![false, false, false],
        vec![true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone(true).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
            vec![true, true, true],
            vec![false, false, false],
            vec![true, true, true],
        ]
    )
}

#[test]
fn population_size_one() {
    let genotype = BinaryGenotype::builder().with_gene_size(5).build().unwrap();

    let population = &mut build::population(vec![vec![true, false, true, false, true]]);

    let mut rng = SmallRng::seed_from_u64(0);
    CrossoverClone(false).call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![vec![true, false, true, false, true]]
    )
}

#[test]
fn allow_unique_genotype() {
    assert_eq!(CrossoverClone(false).allow_unique_genotype(), true);
    assert_eq!(CrossoverClone(true).allow_unique_genotype(), true);
}
