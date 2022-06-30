#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype, IncrementalGenotype};

#[test]
fn general_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 0.9798802, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509
        ]
    );

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 0.9763819, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509
        ]
    );

    assert_eq!(
        genotype.crossover_indexes(),
        (0..10).collect::<Vec<usize>>()
    );
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<usize>>());
}

#[test]
fn general_neighbour() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 0.9798802, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509
        ]
    );

    genotype.mutate_chromosome_neighbour(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 1.0, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509
        ]
    );

    assert_eq!(
        genotype.crossover_indexes(),
        (0..10).collect::<Vec<usize>>()
    );
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<usize>>());
}

#[test]
fn chromosome_neighbours_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![0.447325]);

    assert_eq!(genotype.chromosome_neighbours_size(), BigUint::from(2u32));
    let chromosomes = genotype.chromosome_neighbours(&chromosome, 1.0);
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![vec![0.347325], vec![0.547325],]
    );

    let chromosomes = genotype.chromosome_neighbours(&chromosome, 0.5);
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![vec![0.39732498], vec![0.497325]]
    );
}

#[test]
fn chromosome_neighbours_2() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![0.447325, 0.4391402]);

    assert_eq!(genotype.chromosome_neighbours_size(), BigUint::from(4u32));
    let chromosomes = genotype.chromosome_neighbours(&chromosome, 1.0);
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0.347325, 0.4391402],
            vec![0.547325, 0.4391402],
            vec![0.447325, 0.3391402],
            vec![0.447325, 0.5391402],
        ]
    );
}

#[test]
fn chromosome_neighbours_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(-0.1..0.1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0.447325, 0.4391402, 0.9798802]
    );

    assert_eq!(genotype.chromosome_neighbours_size(), BigUint::from(6u32));
    let chromosomes = genotype.chromosome_neighbours(&chromosome, 1.0);
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0.347325, 0.4391402, 0.9798802],
            vec![0.547325, 0.4391402, 0.9798802],
            vec![0.447325, 0.3391402, 0.9798802],
            vec![0.447325, 0.5391402, 0.9798802],
            vec![0.447325, 0.4391402, 0.8798802],
            vec![0.447325, 0.4391402, 1.0],
        ]
    );
}

#[test]
fn chromosome_neighbours_3_one_sided() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..1.0)
        .with_allele_neighbour_range(0.0..0.1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0.447325, 0.4391402, 0.9798802]
    );

    assert_eq!(genotype.chromosome_neighbours_size(), BigUint::from(3u32));
    let chromosomes = genotype.chromosome_neighbours(&chromosome, 1.0);
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0.547325, 0.4391402, 0.9798802],
            vec![0.447325, 0.5391402, 0.9798802],
            vec![0.447325, 0.4391402, 1.0],
        ]
    );
}
