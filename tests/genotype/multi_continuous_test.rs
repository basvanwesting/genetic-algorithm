#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, MultiContinuousGenotype};

#[test]
fn general() {
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

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0.447325, 2.1957011, 18.970789]
    );

    assert_eq!(genotype.is_unique(), false);
}
