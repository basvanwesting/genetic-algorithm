#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, HillClimbGenotype, PermutateGenotype, UniqueGenotype};

#[test]
fn sample_gene_indices() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![10, 11, 12, 13, 14, 15, 16, 17, 18, 19])
        .build()
        .unwrap();

    assert_eq!(
        genotype.sample_gene_indices(10, false, &mut rng),
        vec![5, 0, 8, 9, 7, 2, 4, 1, 3, 6]
    );
    assert_eq!(
        genotype.sample_gene_indices(10, true, &mut rng),
        vec![5, 1, 2, 8, 3, 9, 9, 0, 8, 4]
    );
}
#[test]
fn mutate_chromosome_single() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![5, 2, 3, 4])
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 2, 3]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 3, 2]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 5, 3, 4]);
}
#[test]
fn mutate_chromosome_genes_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    genotype.mutate_chromosome_genes(3, true, &mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![1, 2, 3, 5, 9, 6, 7, 8, 4]
    );
}
#[test]
fn mutate_chromosome_genes_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    genotype.mutate_chromosome_genes(3, false, &mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![3, 2, 1, 4, 7, 8, 5, 6, 9]
    );
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
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None)
                .collect::<Vec<_>>()
                .as_slice()
        ),
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

    // ensure lazy
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None)
                .take(1)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29
        ]]
    )
}

#[test]
fn neighbouring_population_2() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![0, 1])
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![0, 1]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(1u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(inspect::population(&population), vec![vec![1, 0]]);
}
#[test]
fn neighbouring_population_4() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueGenotype::builder()
        .with_allele_list(vec![0, 1, 2, 3])
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![3, 0, 1, 2]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));

    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
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
