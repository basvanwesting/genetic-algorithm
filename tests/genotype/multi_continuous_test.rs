#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, MultiContinuousGenotype};

#[test]
fn general_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_multi_range(vec![0.0..1.0, 0.0..5.0, 10.0..20.0])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 19.798801]
    );

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 18.970789]
    );

    // ensure default implementation when neighbour ranges not provided
    genotype.mutate_chromosome_neighbour(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 20.0]
    );

    assert_eq!(genotype.crossover_indexes(), (0..3).collect::<Vec<usize>>());
    assert_eq!(genotype.crossover_points(), (0..3).collect::<Vec<usize>>());
}

#[test]
fn general_neighbour() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_multi_range(vec![0.0..1.0, 0.0..5.0, 10.0..20.0])
        .with_allele_multi_neighbour_range(vec![-1.0..0.1, -0.5..0.5, -1.0..1.0])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 19.798801]
    );

    genotype.mutate_chromosome_neighbour(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 20.0]
    );

    assert_eq!(genotype.crossover_indexes(), (0..3).collect::<Vec<usize>>());
    assert_eq!(genotype.crossover_points(), (0..3).collect::<Vec<usize>>());
}
