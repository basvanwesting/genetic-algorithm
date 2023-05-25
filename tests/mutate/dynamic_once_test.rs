#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
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
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);

    let mut rng = SmallRng::seed_from_u64(0);
    let mut mutate = Mutate::DynamicOnce {
        mutation_probability: 0.0,
        mutation_probability_step: 0.1,
        target_uniformity: 0.9,
    };
    let mut fitness = CountTrue;
    assert_eq!(mutate.report(), "prob: 0.00");
    fitness.call_for_population_single_thread(population);
    mutate.call(&genotype, population, &mut rng);
    assert_eq!(mutate.report(), "prob: 0.10");
    fitness.call_for_population_single_thread(population);
    mutate.call(&genotype, population, &mut rng);
    assert_eq!(mutate.report(), "prob: 0.20");
    fitness.call_for_population_single_thread(population);
    mutate.call(&genotype, population, &mut rng);
    assert_eq!(mutate.report(), "prob: 0.10");
    fitness.call_for_population_single_thread(population);
    mutate.call(&genotype, population, &mut rng);
    assert_eq!(mutate.report(), "prob: 0.10");

    assert_eq!(
        inspect::population(population),
        vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, false],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, false, true],
        ]
    );
}
