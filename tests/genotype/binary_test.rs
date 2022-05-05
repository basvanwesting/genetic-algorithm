#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype, PermutableGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::new().with_gene_size(10).build();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, false, true, false, false, false, true, true, false]
    );

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, true, true, false, false, false, true, true, false]
    );

    assert_eq!(genotype.gene_values(), vec![true, false]);
    assert_eq!(genotype.population_factory_size(), 1024);
    assert_eq!(genotype.is_unique(), false);
}

#[test]
fn population_factory_size_one() {
    let genotype = BinaryGenotype::new().with_gene_size(1).build();
    let population = genotype.population_factory();
    println!("{:#?}", population);

    assert_eq!(genotype.population_factory_size(), 2);
    assert_eq!(
        inspect::population(&population),
        vec![vec![true], vec![false],]
    )
}

#[test]
fn population_factory_size_two() {
    let genotype = BinaryGenotype::new().with_gene_size(2).build();
    let population = genotype.population_factory();
    println!("{:#?}", population);

    assert_eq!(genotype.population_factory_size(), 4);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true],
            vec![true, false],
            vec![false, true],
            vec![false, false],
        ]
    )
}

#[test]
fn population_factory_size_three() {
    let genotype = BinaryGenotype::new().with_gene_size(3).build();
    let population = genotype.population_factory();
    println!("{:#?}", population);

    assert_eq!(genotype.population_factory_size(), 8);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
        ]
    )
}
