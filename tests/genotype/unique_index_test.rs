#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, PermutableGenotype, UniqueIndexGenotype};

#[test]
fn test() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = UniqueIndexGenotype::new().with_gene_value_size(5).build();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![3, 0, 1, 4, 2]);

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![3, 0, 1, 2, 4]);

    assert_eq!(genotype.gene_values(), vec![0, 1, 2, 3, 4]);
    assert_eq!(genotype.population_factory_size(), 120);
    assert_eq!(genotype.is_unique(), true);
}
