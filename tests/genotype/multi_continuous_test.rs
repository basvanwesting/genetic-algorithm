#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, IncrementalGenotype, MultiContinuousGenotype};

#[test]
fn float_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 19.798801],
        0.001
    ));

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 18.970789],
        0.001
    ));

    assert_eq!(genotype.crossover_indexes(), (0..3).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..3).collect::<Vec<_>>());
}

#[test]
fn float_neighbour() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_neighbour_ranges(vec![-1.0..=0.1, -0.5..=0.5, -1.0..=1.0])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 19.798801],
        0.001
    ));

    genotype.mutate_chromosome_neighbour(&mut chromosome, Some(1.0), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 20.0],
        0.001
    ));

    assert_eq!(genotype.crossover_indexes(), (0..3).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..3).collect::<Vec<_>>());
}

#[test]
fn float_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0])
        .with_allele_neighbour_ranges(vec![-1.0..=0.1])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447325],
        0.001
    ));

    //FIXME: scale doesn't work for generic
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![vec![0.0], vec![0.547325],],
        0.001
    ));
}

#[test]
fn float_neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_neighbour_ranges(vec![-1.0..=0.1, -0.5..=0.5, -1.0..=1.0])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 19.798801],
        0.001
    ));

    //FIXME: scale doesn't work for generic
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![
            vec![0.0, 2.1957011, 19.798801],
            vec![0.547325, 2.1957011, 19.798801],
            vec![0.447325, 1.6957011, 19.798801],
            vec![0.447325, 2.6957011, 19.798801],
            vec![0.447325, 2.1957011, 18.798801],
            vec![0.447325, 2.1957011, 20.0],
        ],
        0.001
    ));
}

#[test]
fn integer_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 20]);

    assert_eq!(genotype.crossover_indexes(), (0..3).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..3).collect::<Vec<_>>());
}

#[test]
fn integer_neighbour() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .with_allele_neighbour_ranges(vec![-1..=1, -2..=2, -3..=3])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    //FIXME: scale doesn't work for generic
    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 20]);

    assert_eq!(genotype.crossover_indexes(), (0..3).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..3).collect::<Vec<_>>());
}

#[test]
fn integer_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![0..=9])
        .with_allele_neighbour_ranges(vec![-1..=1])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![vec![3], vec![5]],
    );
}

#[test]
fn integer_neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .with_allele_neighbour_ranges(vec![-1..=1, -2..=2, -3..=3])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![
            vec![3, 2, 20],
            vec![5, 2, 20],
            vec![4, 0, 20],
            vec![4, 4, 20],
            vec![4, 2, 17],
            vec![4, 2, 20]
        ]
    );
}
