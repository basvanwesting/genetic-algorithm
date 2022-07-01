#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    Genotype, IncrementalGenotype, PermutableGenotype, UniqueGenotype,
};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![5, 2, 3, 4])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 2, 3]);

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 3, 2]);

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(24u32)
    );
    assert_eq!(genotype.crossover_indexes(), vec![]);
    assert_eq!(genotype.crossover_points(), vec![]);
}

#[test]
fn chromosome_permutations() {
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![0, 1, 2, 3])
        .build()
        .unwrap();

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(24u32)
    );
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
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

#[test]
fn chromosome_permutations_genes_size_huge() {
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..30).collect())
        .build()
        .unwrap();
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::parse_bytes(b"265252859812191058636308480000000", 10).unwrap()
    );
}

#[test]
fn chromosome_neighbours_2() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![0, 1])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![0, 1]);

    assert_eq!(genotype.chromosome_neighbours_size(), BigUint::from(1u32));
    let chromosomes = genotype.chromosome_neighbours(&chromosome, None);
    assert_eq!(inspect::chromosomes(&chromosomes), vec![vec![1, 0]]);
}
#[test]
fn chromosome_neighbours_4() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![0, 1, 2, 3])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![3, 0, 1, 2]);

    assert_eq!(genotype.chromosome_neighbours_size(), BigUint::from(6u32));
    let chromosomes = genotype.chromosome_neighbours(&chromosome, None);
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0, 3, 1, 2],
            vec![1, 0, 3, 2],
            vec![2, 0, 1, 3],
            vec![3, 1, 0, 2],
            vec![3, 2, 1, 0],
            vec![3, 0, 2, 1],
        ]
    );
}
