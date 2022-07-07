#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    Genotype, IncrementalGenotype, MultiDiscreteGenotype, PermutableGenotype,
};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0, 1, 2],
            vec![4, 5, 6, 7],
        ])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 5]);

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 7]);

    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 1, 7]);

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(120u32)
    );
    assert_eq!(genotype.crossover_indexes(), (0..4).collect::<Vec<usize>>());
    assert_eq!(genotype.crossover_points(), (0..4).collect::<Vec<usize>>());
}

#[test]
fn chromosome_neighbours_size() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![0, 0, 2, 1]);

    assert_eq!(genotype.chromosome_neighbours_size(), BigUint::from(6u32));
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_neighbours(&chromosome, None)),
        vec![
            vec![0, 1, 2, 1],
            vec![0, 0, 0, 1],
            vec![0, 0, 1, 1],
            vec![0, 0, 2, 0],
            vec![0, 0, 2, 2],
            vec![0, 0, 2, 3],
        ]
    );
}

#[test]
fn chromosome_permutations_genes_size_1() {
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(vec![vec![0]])
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(1u32));
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![vec![0]]
    );
}

#[test]
fn chromosome_permutations_genes_size_4() {
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]])
        .build()
        .unwrap();

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(24u32)
    );
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

#[test]
fn chromosome_permutations_genes_size_huge() {
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(vec![
            (0..1000).collect(),
            (0..1000).collect(),
            (0..1000).collect(),
            (0..1000).collect(),
            (0..1000).collect(),
            (0..1000).collect(),
            (0..1000).collect(),
            (0..1000).collect(),
            (0..1000).collect(),
            (0..1000).collect(),
        ])
        .build()
        .unwrap();
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::parse_bytes(b"1000000000000000000000000000000", 10).unwrap()
    );
}
