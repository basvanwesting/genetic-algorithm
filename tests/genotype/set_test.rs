#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, PermutableGenotype, SetGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = SetGenotype::builder()
        .with_gene_values(vec![2, 3, 4, 5])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![5, 2, 3]);
    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![5, 2, 3, 4]);
    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![5, 2, 3, 4]);
    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![5, 4, 3]);

    assert_eq!(genotype.gene_values(), vec![2, 3, 4, 5]);
    assert_eq!(genotype.is_unique(), true);
}

#[test]
fn chromosome_permutations() {
    let genotype = SetGenotype::builder()
        .with_gene_values(vec![0, 1, 2, 3])
        .build()
        .unwrap();

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(15u32)
    );
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![
            vec![],
            vec![0],
            vec![1],
            vec![2],
            vec![3],
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![0, 1, 2],
            vec![0, 1, 3],
            vec![0, 2, 3],
            vec![1, 2, 3],
        ]
    );
}

#[test]
fn chromosome_permutations_gene_size_huge() {
    let genotype = SetGenotype::builder()
        .with_gene_values((0..50).collect())
        .build()
        .unwrap();
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::parse_bytes(b"1125899906842623", 10).unwrap()
    );
}
