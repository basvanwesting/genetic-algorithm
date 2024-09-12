#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::genotype::{Genotype, IncrementalGenotype, MultiRangeGenotype};

#[test]
fn float_mutate_chromosome_single_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.195, 19.798],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.195, 18.970],
        0.001
    ));
}

#[test]
fn float_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_mutation_ranges(vec![-0.1..=0.1, -0.5..=0.5, -1.0..=1.0])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 19.798],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 20.0],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 19.790],
        0.001
    ));
}

#[test]
fn float_mutate_chromosome_single_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_mutation_scaled_ranges(vec![
            vec![-0.5..=0.5, -1.0..=1.0, -5.0..=5.0],
            vec![-0.1..=0.1, -0.5..=0.5, -1.0..=1.0],
            vec![-0.01..=0.01, -0.05..=0.05, -0.1..=0.1],
        ])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.195, 19.798],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.195, 19.698],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.195, 19.598],
        0.001
    ));
}

#[test]
fn mutate_chromosome_genes_random_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0.0, 0.0, 10.0]);
    genotype.mutate_chromosome_genes(3, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.0, 0.0, 19.429],
        0.001
    ));
}
#[test]
fn mutate_chromosome_genes_random_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0.0, 0.0, 10.0]);
    genotype.mutate_chromosome_genes(2, false, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.0, 4.485, 14.621],
        0.001
    ));
}

#[test]
fn float_crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0.1, 1.1, 10.1]);
    let mut mother = build::chromosome(vec![0.9, 3.9, 18.9]);
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome(&father), vec![0.1, 3.9, 10.1]);
    assert_eq!(inspect::chromosome(&mother), vec![0.9, 1.1, 18.9]);
}

#[test]
fn float_crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0.1, 1.1, 10.1]);
    let mut mother = build::chromosome(vec![0.9, 3.9, 18.9]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome(&father), vec![0.1, 3.9, 18.9]);
    assert_eq!(inspect::chromosome(&mother), vec![0.9, 1.1, 10.1]);
}

#[test]
fn float_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0])
        .with_allele_mutation_ranges(vec![-0.1..=0.1])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![vec![0.391], vec![0.545]],
        0.001
    ));
}

#[test]
fn float_neighbouring_population_3_unscaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_mutation_ranges(vec![-0.1..=0.1, -0.5..=0.5, -1.0..=1.0])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 19.798],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.394, 2.196, 19.799],
            vec![0.537, 2.196, 19.799],
            vec![0.447, 2.167, 19.799],
            vec![0.447, 2.490, 19.799],
            vec![0.447, 2.196, 19.255],
            vec![0.447, 2.196, 19.878],
        ],
        0.001
    ));
}

#[test]
fn float_neighbouring_population_3_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_mutation_scaled_ranges(vec![
            vec![-0.5..=0.5, -1.0..=1.0, -5.0..=5.0],
            vec![-0.1..=0.1, -0.5..=0.5, -1.0..=1.0],
            vec![-0.01..=0.01, -0.05..=0.05, -0.1..=0.1],
        ])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 19.798],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(0), &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.0, 2.196, 19.799],
            vec![0.947, 2.196, 19.799],
            vec![0.447, 1.196, 19.799],
            vec![0.447, 3.196, 19.799],
            vec![0.447, 2.196, 14.799],
            vec![0.447, 2.196, 20.0],
        ],
        0.001
    ));
    genotype.chromosome_destructor_truncate(&mut population.chromosomes, 0);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(1), &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.347, 2.196, 19.799],
            vec![0.547, 2.196, 19.799],
            vec![0.447, 1.696, 19.799],
            vec![0.447, 2.696, 19.799],
            vec![0.447, 2.196, 18.799],
            vec![0.447, 2.196, 20.0]
        ],
        0.001
    ));
    genotype.chromosome_destructor_truncate(&mut population.chromosomes, 0);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(2), &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.437, 2.196, 19.799],
            vec![0.457, 2.196, 19.799],
            vec![0.447, 2.146, 19.799],
            vec![0.447, 2.246, 19.799],
            vec![0.447, 2.196, 19.699],
            vec![0.447, 2.196, 19.899],
        ],
        0.001
    ));
}

#[test]
fn integer_mutate_chromosome_single_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 20]);
}

#[test]
fn integer_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .with_allele_mutation_ranges(vec![-1..=1, -2..=2, -3..=3])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 20]);
}

#[test]
fn integer_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0..=9])
        .with_allele_mutation_ranges(vec![-1..=1])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(inspect::population(&population), vec![vec![3], vec![5]],);
}

#[test]
fn integer_neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .with_allele_mutation_ranges(vec![-1..=1, -2..=2, -3..=3])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![3, 2, 20],
            vec![5, 2, 20],
            vec![4, 0, 20],
            vec![4, 4, 20],
            vec![4, 2, 17],
        ]
    );
}
