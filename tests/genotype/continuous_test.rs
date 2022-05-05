#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype};

#[test]
fn test() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::new().with_gene_size(10).build();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.43914026, 0.9798802, 0.4621672, 0.897079, 0.9429498, 0.58814746,
            0.45637196, 0.39514416, 0.81885093
        ]
    );

    genotype.mutate_chromosome(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.43914026, 0.9763819, 0.4621672, 0.897079, 0.9429498, 0.58814746,
            0.45637196, 0.39514416, 0.81885093
        ]
    );

    assert_eq!(genotype.is_unique(), false);
}
