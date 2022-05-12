#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, MultiDiscreteGenotype, PermutableGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiDiscreteGenotype::builder()
        .with_gene_multi_values(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0, 1, 2],
            vec![4, 5, 6, 7],
        ])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 5]);

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 7]);

    assert_eq!(genotype.chromosome_permutations_size(), 120);
    assert_eq!(genotype.is_unique(), false);
}

#[test]
fn chromosome_permutations_gene_size_1() {
    let genotype = MultiDiscreteGenotype::builder()
        .with_gene_multi_values(vec![vec![0]])
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), 1);
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![vec![0]]
    );
}

#[test]
fn chromosome_permutations_gene_size_4() {
    let genotype = MultiDiscreteGenotype::builder()
        .with_gene_multi_values(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]])
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), 24);
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
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
