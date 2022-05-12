#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{DiscreteGenotype, Genotype, PermutableGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = DiscreteGenotype::builder()
        .with_gene_size(5)
        .with_gene_values(vec![5, 2, 3, 4])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 4]);

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 3]);

    assert_eq!(genotype.chromosome_permutations_size(), 1024);
    assert_eq!(genotype.is_unique(), false);
}

#[test]
fn chromosome_permutations() {
    let genotype = DiscreteGenotype::builder()
        .with_gene_size(3)
        .with_gene_values(vec![0, 1, 2])
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), 27);
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
