#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    ListGenotype, Genotype, IncrementalGenotype, PermutableGenotype,
};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list(vec![5, 2, 3, 4])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 4]);

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 3]);

    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 2, 2, 3]);

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(1024u32)
    );
    assert_eq!(genotype.crossover_indexes(), (0..5).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..5).collect::<Vec<_>>());
}

#[test]
fn neighbouring_population() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list(vec![5, 2, 3, 4])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 4]);

    assert_eq!(
        genotype.neighbouring_population_size(),
        BigUint::from(15u32)
    );
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
        vec![
            vec![5, 2, 4, 2, 4],
            vec![3, 2, 4, 2, 4],
            vec![4, 2, 4, 2, 4],
            vec![2, 5, 4, 2, 4],
            vec![2, 3, 4, 2, 4],
            vec![2, 4, 4, 2, 4],
            vec![2, 2, 5, 2, 4],
            vec![2, 2, 2, 2, 4],
            vec![2, 2, 3, 2, 4],
            vec![2, 2, 4, 5, 4],
            vec![2, 2, 4, 3, 4],
            vec![2, 2, 4, 4, 4],
            vec![2, 2, 4, 2, 5],
            vec![2, 2, 4, 2, 2],
            vec![2, 2, 4, 2, 3],
        ]
    );
}

#[test]
fn chromosome_permutations() {
    let genotype = ListGenotype::builder()
        .with_genes_size(3)
        .with_allele_list(vec![0, 1, 2])
        .build()
        .unwrap();

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(27u32)
    );
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 0, 2],
            vec![0, 1, 0],
            vec![0, 1, 1],
            vec![0, 1, 2],
            vec![0, 2, 0],
            vec![0, 2, 1],
            vec![0, 2, 2],
            vec![1, 0, 0],
            vec![1, 0, 1],
            vec![1, 0, 2],
            vec![1, 1, 0],
            vec![1, 1, 1],
            vec![1, 1, 2],
            vec![1, 2, 0],
            vec![1, 2, 1],
            vec![1, 2, 2],
            vec![2, 0, 0],
            vec![2, 0, 1],
            vec![2, 0, 2],
            vec![2, 1, 0],
            vec![2, 1, 1],
            vec![2, 1, 2],
            vec![2, 2, 0],
            vec![2, 2, 1],
            vec![2, 2, 2],
        ]
    );
}

#[test]
fn chromosome_permutations_genes_size_huge() {
    let genotype = ListGenotype::builder()
        .with_genes_size(30)
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::parse_bytes(b"1000000000000000000000000000000", 10).unwrap()
    );
}
