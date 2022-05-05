#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, IndexGenotype, PermutableGenotype};

#[test]
fn test() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = IndexGenotype::new()
        .with_gene_size(10)
        .with_gene_value_size(5)
        .build();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![2, 2, 4, 2, 4, 4, 2, 2, 1, 4]
    );

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![2, 2, 4, 2, 4, 4, 0, 2, 1, 4]
    );

    assert_eq!(genotype.gene_values(), vec![0, 1, 2, 3, 4]);
    assert_eq!(genotype.population_factory_size(), 9_765_625);
    assert_eq!(genotype.is_unique(), false);
}
