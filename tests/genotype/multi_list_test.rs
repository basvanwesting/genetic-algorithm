#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::genotype::{
    EvolveGenotype, Genotype, HillClimbGenotype, MultiListGenotype, PermutableGenotype,
};

#[test]
fn mutate_chromosome_single() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0, 1, 2],
            vec![4, 5, 6, 7],
        ])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 5]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 2, 7]);

    // genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    // assert_eq!(inspect::chromosome(&chromosome), vec![2, 0, 1, 7]);
}
#[test]
fn mutate_chromosome_genes_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![9, 8, 7],
            vec![6, 5, 4],
            vec![3, 2, 1],
        ])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0, 3, 6, 9, 6, 3]);
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![0, 3, 8, 8, 6, 1]);
}
#[test]
fn mutate_chromosome_genes_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![9, 8, 7],
            vec![6, 5, 4],
            vec![3, 2, 1],
        ])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0, 3, 6, 9, 6, 3]);
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![0, 3, 7, 7, 5, 2]);
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0, 1, 2],
            vec![4, 5, 6, 7],
        ])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0, 1, 2, 4]);
    let mut mother = build::chromosome(vec![3, 0, 1, 6]);
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome(&father), vec![0, 0, 2, 4]);
    assert_eq!(inspect::chromosome(&mother), vec![3, 1, 1, 6]);
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0, 1, 2],
            vec![4, 5, 6, 7],
        ])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0, 1, 2, 4]);
    let mut mother = build::chromosome(vec![3, 0, 1, 6]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome(&father), vec![0, 0, 1, 6]);
    assert_eq!(inspect::chromosome(&mother), vec![3, 1, 2, 4]);
}

#[test]
fn neighbouring_population_size() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![0, 0, 2, 1]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
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
    let genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![vec![0]])
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(1u32));
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter()
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![vec![0]]
    );
}

#[test]
fn chromosome_permutations_genes_size_4() {
    let genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1, 2, 3]])
        .build()
        .unwrap();

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(24u32)
    );
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter()
                .collect::<Vec<_>>()
                .as_slice()
        ),
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
    let genotype = MultiListGenotype::builder()
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

    // ensure lazy
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter()
                .take(1)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![vec![0; 10]]
    )
}
