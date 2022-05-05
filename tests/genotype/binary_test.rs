#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype, PermutableGenotype};

#[test]
fn test() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::new().with_gene_size(10).build();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, false, true, false, false, false, true, true, false]
    );

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, true, true, false, false, false, true, true, false]
    );

    assert_eq!(genotype.gene_values(), vec![true, false]);
    assert_eq!(genotype.population_factory_size(), 1024);
    assert_eq!(genotype.is_unique(), false);
}
