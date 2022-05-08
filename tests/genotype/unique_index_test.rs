#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, PermutableGenotype, UniqueIndexGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueIndexGenotype::builder()
        .with_gene_value_size(5)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![3, 0, 1, 4, 2]);

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![3, 0, 1, 2, 4]);

    assert_eq!(genotype.gene_values(), vec![0, 1, 2, 3, 4]);
    assert_eq!(genotype.population_factory_size(), 120);
    assert_eq!(genotype.is_unique(), true);
}

#[test]
fn population_factory_size_three() {
    let genotype = UniqueIndexGenotype::builder()
        .with_gene_value_size(3)
        .build()
        .unwrap();
    let population = genotype.population_factory();
    println!("{:#?}", population);

    assert_eq!(genotype.population_factory_size(), 6);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ]
    )
}
