#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, PermutableGenotype, UniqueDiscreteGenotype};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueDiscreteGenotype::new()
        .with_gene_values(vec![5, 2, 3, 4])
        .build();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 2, 3]);

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 3, 2]);

    assert_eq!(genotype.gene_values(), vec![5, 2, 3, 4]);
    assert_eq!(genotype.population_factory_size(), 24);
    assert_eq!(genotype.is_unique(), true);
}
