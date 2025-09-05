#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::{
    EvolveGenotype, Genotype, HillClimbGenotype, ListGenotype, PermutateGenotype,
};

#[test]
fn mutate_chromosome_single() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list(vec![5, 2, 3, 4])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 4]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 3]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 2, 2, 3]);
}
#[test]
fn mutate_chromosome_genes_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = ListGenotype::builder()
        .with_genes_size(10)
        .with_allele_list(vec![1, 2, 3, 4])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![1; 10]);
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![1, 1, 1, 4, 2, 2, 1, 1, 4, 2]
    );
}
#[test]
fn mutate_chromosome_genes_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = ListGenotype::builder()
        .with_genes_size(10)
        .with_allele_list(vec![1, 2, 3, 4])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![1; 10]);
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![1, 1, 2, 4, 3, 1, 1, 1, 4, 1]
    );
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list(vec![5, 2, 3, 4])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![2, 2, 3, 3, 4]);
    let mut mother = build::chromosome(vec![5, 5, 4, 4, 3]);
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome(&father), vec![2, 2, 4, 3, 4]);
    assert_eq!(inspect::chromosome(&mother), vec![5, 5, 3, 4, 3]);
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list(vec![5, 2, 3, 4])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![2, 2, 3, 3, 4]);
    let mut mother = build::chromosome(vec![5, 5, 4, 4, 3]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome(&father), vec![2, 2, 4, 4, 3]);
    assert_eq!(inspect::chromosome(&mother), vec![5, 5, 3, 3, 4]);
}

#[test]
fn neighbouring_population() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list(vec![5, 2, 3, 4])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 4]);

    assert_eq!(
        genotype.neighbouring_population_size(),
        BigUint::from(15u32)
    );
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
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
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .collect::<Vec<_>>()
                .as_slice()
        ),
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

    // ensure lazy
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .take(1)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![vec![0; 30]]
    )
}

#[test]
fn integer_calculate_genes_hash() {
    let mut genotype = ListGenotype::builder()
        .with_genes_size(10)
        .with_allele_list((-10..10).collect())
        .with_genes_hashing(true)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome_1 = build::chromosome(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_2 = build::chromosome(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_3 = build::chromosome(vec![-0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_4 = build::chromosome(vec![-0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert!(genotype.calculate_genes_hash(&chromosome_1).is_some());
    // assert_eq!(
    //     genotype.calculate_genes_hash(&chromosome_1),
    //     Some(10873053262589934868)
    // );
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_1),
        genotype.calculate_genes_hash(&chromosome_2),
    );
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_3),
        genotype.calculate_genes_hash(&chromosome_4),
    );

    // the sign on does not matter
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_1),
        genotype.calculate_genes_hash(&chromosome_3),
    );
}
