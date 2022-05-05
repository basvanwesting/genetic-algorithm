#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{DiscreteGenotype, Genotype, PermutableGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = DiscreteGenotype::new()
        .with_gene_size(5)
        .with_gene_values(vec![5, 2, 3, 4])
        .build();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 4]);

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![2, 2, 4, 2, 3]);

    assert_eq!(genotype.population_factory_size(), 1024);
    assert_eq!(genotype.is_unique(), false);
}
