#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, MultiIndexGenotype, PermutableGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiIndexGenotype::new()
        .with_gene_value_sizes(vec![5, 2, 3, 4])
        .build();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 1]);

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 3]);

    assert_eq!(genotype.population_factory_size(), 120);
    assert_eq!(genotype.is_unique(), false);
}

#[test]
fn population_factory_size_one() {
    let genotype = MultiIndexGenotype::new()
        .with_gene_value_sizes(vec![1])
        .build();

    assert_eq!(genotype.population_factory_size(), 1);
    assert_eq!(
        inspect::population(&genotype.population_factory()),
        vec![vec![0]]
    );
}

#[test]
fn population_factory_size_four() {
    let genotype = MultiIndexGenotype::new()
        .with_gene_value_sizes(vec![1, 2, 3, 4])
        .build();

    assert_eq!(genotype.population_factory_size(), 24);
    assert_eq!(
        inspect::population(&genotype.population_factory()),
        vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 2],
            vec![0, 0, 0, 3],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 2],
            vec![0, 0, 1, 3],
            vec![0, 0, 2, 0],
            vec![0, 0, 2, 1],
            vec![0, 0, 2, 2],
            vec![0, 0, 2, 3],
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 1],
            vec![0, 1, 0, 2],
            vec![0, 1, 0, 3],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 1],
            vec![0, 1, 1, 2],
            vec![0, 1, 1, 3],
            vec![0, 1, 2, 0],
            vec![0, 1, 2, 1],
            vec![0, 1, 2, 2],
            vec![0, 1, 2, 3],
        ]
    );
}
