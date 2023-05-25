#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{BinaryGenotype, DiscreteGenotype, Genotype};
use genetic_algorithm::mutate::Mutate;

#[test]
fn binary_genotype() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    let population = &mut build::population(vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    Mutate::Once {
        mutation_probability: 0.5,
    }
    .call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![
            vec![true, false, true],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, true],
        ]
    );
}

#[test]
fn discrete_genotype() {
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(3)
        .with_allele_list(vec![0, 1, 2, 3])
        .build()
        .unwrap();

    let population = &mut build::population(vec![
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
        vec![0, 0, 0],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    Mutate::Once {
        mutation_probability: 0.5,
    }
    .call(&genotype, population, &mut rng);

    assert_eq!(
        inspect::population(population),
        vec![vec![0, 3, 0], vec![0, 0, 3], vec![0, 0, 0], vec![0, 3, 0],]
    );
}
