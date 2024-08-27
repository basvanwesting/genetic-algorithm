#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    Genotype, IncrementalGenotype, MultiUniqueGenotype, PermutableGenotype,
};

#[test]
fn mutate_chomosome() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0, 1], vec![4, 5, 6, 7], vec![0, 1, 2]])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 1, 6, 5, 4, 7, 1, 2, 0]
    );

    genotype.mutate_chromosome(&mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 1, 6, 7, 4, 5, 1, 2, 0]
    );

    genotype.mutate_chromosome(&mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 1, 6, 7, 4, 5, 2, 1, 0]
    );

    genotype.mutate_chromosome(&mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 1, 6, 5, 4, 7, 2, 1, 0]
    );

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(288u32)
    );
}

#[test]
#[should_panic]
fn crossover_chromosome_pair_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0, 1], vec![4, 5, 6, 7], vec![0, 1, 2]])
        .build()
        .unwrap();

    assert_eq!(genotype.crossover_indexes(), vec![]);
    let mut father = build::chromosome(vec![0, 1, 4, 5, 6, 7, 0, 1, 2]);
    let mut mother = build::chromosome(vec![1, 0, 5, 6, 7, 4, 1, 2, 0]);
    genotype.crossover_chromosome_pair_gene(&mut father, &mut mother, rng);
}

#[test]
fn crossover_chromosome_pair_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0, 1], vec![4, 5, 6, 7], vec![0, 1, 2]])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_list_sizes, vec![2, 4, 3]);
    assert_eq!(genotype.allele_list_index_offsets, vec![0, 2, 6, 9]);
    assert_eq!(genotype.crossover_points(), vec![2, 6]);
    let mut father = build::chromosome(vec![0, 1, 4, 5, 6, 7, 0, 1, 2]);
    let mut mother = build::chromosome(vec![1, 0, 5, 6, 7, 4, 1, 2, 0]);
    genotype.crossover_chromosome_pair_point(&mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![0, 1, 5, 6, 7, 4, 1, 2, 0]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![1, 0, 4, 5, 6, 7, 0, 1, 2]
    );
}

#[test]
fn chromosome_permutations_genes_size_1() {
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0]])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_list_sizes, vec![1]);
    assert_eq!(genotype.allele_list_index_offsets, vec![0, 1]);
    assert_eq!(genotype.crossover_points, vec![]);
    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(1u32));
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![vec![0]]
    );
}

#[test]
fn chromosome_permutations_genes_size_4() {
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1]])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_list_sizes, vec![1, 2, 3, 2]);
    assert_eq!(genotype.allele_list_index_offsets, vec![0, 1, 3, 6, 8]);
    assert_eq!(genotype.crossover_points, vec![1, 3, 6]);
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(24u32)
    );
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![
            vec![0, 0, 1, 0, 1, 2, 0, 1],
            vec![0, 0, 1, 0, 1, 2, 1, 0],
            vec![0, 0, 1, 0, 2, 1, 0, 1],
            vec![0, 0, 1, 0, 2, 1, 1, 0],
            vec![0, 0, 1, 1, 0, 2, 0, 1],
            vec![0, 0, 1, 1, 0, 2, 1, 0],
            vec![0, 0, 1, 1, 2, 0, 0, 1],
            vec![0, 0, 1, 1, 2, 0, 1, 0],
            vec![0, 0, 1, 2, 0, 1, 0, 1],
            vec![0, 0, 1, 2, 0, 1, 1, 0],
            vec![0, 0, 1, 2, 1, 0, 0, 1],
            vec![0, 0, 1, 2, 1, 0, 1, 0],
            vec![0, 1, 0, 0, 1, 2, 0, 1],
            vec![0, 1, 0, 0, 1, 2, 1, 0],
            vec![0, 1, 0, 0, 2, 1, 0, 1],
            vec![0, 1, 0, 0, 2, 1, 1, 0],
            vec![0, 1, 0, 1, 0, 2, 0, 1],
            vec![0, 1, 0, 1, 0, 2, 1, 0],
            vec![0, 1, 0, 1, 2, 0, 0, 1],
            vec![0, 1, 0, 1, 2, 0, 1, 0],
            vec![0, 1, 0, 2, 0, 1, 0, 1],
            vec![0, 1, 0, 2, 0, 1, 1, 0],
            vec![0, 1, 0, 2, 1, 0, 0, 1],
            vec![0, 1, 0, 2, 1, 0, 1, 0],
        ]
    );
}

#[test]
fn chromosome_permutations_genes_size_huge() {
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![
            (0..10).collect(),
            (0..10).collect(),
            (0..10).collect(),
            (0..10).collect(),
            (0..10).collect(),
            (0..10).collect(),
        ])
        .build()
        .unwrap();
    assert_eq!(genotype.allele_list_sizes, vec![10, 10, 10, 10, 10, 10]);
    assert_eq!(
        genotype.allele_list_index_offsets,
        vec![0, 10, 20, 30, 40, 50, 60]
    );
    assert_eq!(genotype.crossover_points, vec![10, 20, 30, 40, 50]);
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::parse_bytes(b"2283380023591730815784976384000000000000", 10).unwrap()
    );

    // ensure lazy
    assert_eq!(
        inspect::chromosomes(
            &genotype
                .chromosome_permutations_into_iter()
                .take(1)
                .collect()
        ),
        vec![vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8,
            9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
            8, 9,
        ]]
    )
}

#[test]
fn neighbouring_population_4() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1]])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 0, 1, 2, 0, 1, 0, 1]
    );

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(5u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
        vec![
            vec![0, 1, 0, 2, 0, 1, 0, 1],
            vec![0, 0, 1, 0, 2, 1, 0, 1],
            vec![0, 0, 1, 1, 0, 2, 0, 1],
            vec![0, 0, 1, 2, 1, 0, 0, 1],
            vec![0, 0, 1, 2, 0, 1, 1, 0]
        ]
    );
}
