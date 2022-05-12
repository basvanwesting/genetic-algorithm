#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, PermutableGenotype, UniqueDiscreteGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueDiscreteGenotype::builder()
        .with_gene_values(vec![5, 2, 3, 4])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 2, 3]);

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 3, 2]);

    assert_eq!(genotype.gene_values(), vec![5, 2, 3, 4]);
    assert_eq!(genotype.population_factory_size(), 24);
    assert_eq!(genotype.is_unique(), true);
}

#[test]
fn population_factory() {
    let genotype = UniqueDiscreteGenotype::builder()
        .with_gene_values(vec![0, 1, 2, 3])
        .build()
        .unwrap();

    assert_eq!(genotype.population_factory_size(), 24);
    assert_eq!(
        inspect::population(&genotype.population_factory()),
        vec![
            vec![0, 1, 2, 3],
            vec![0, 1, 3, 2],
            vec![0, 2, 1, 3],
            vec![0, 2, 3, 1],
            vec![0, 3, 1, 2],
            vec![0, 3, 2, 1],
            vec![1, 0, 2, 3],
            vec![1, 0, 3, 2],
            vec![1, 2, 0, 3],
            vec![1, 2, 3, 0],
            vec![1, 3, 0, 2],
            vec![1, 3, 2, 0],
            vec![2, 0, 1, 3],
            vec![2, 0, 3, 1],
            vec![2, 1, 0, 3],
            vec![2, 1, 3, 0],
            vec![2, 3, 0, 1],
            vec![2, 3, 1, 0],
            vec![3, 0, 1, 2],
            vec![3, 0, 2, 1],
            vec![3, 1, 0, 2],
            vec![3, 1, 2, 0],
            vec![3, 2, 0, 1],
            vec![3, 2, 1, 0],
        ]
    );
}
