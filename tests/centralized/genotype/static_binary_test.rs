#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::{
    EvolveGenotype, Genotype, HillClimbGenotype, StaticBinaryGenotype,
};

#[test]
fn chromosome_constructor() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(
        genotype.genes_slice(&chromosome),
        &[false, false, true, false, true, true, true, false, false, true]
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        genotype.genes_slice(&chromosome),
        &[false, false, false, false, true, true, true, false, false, true]
    );
}

#[test]
fn mutate_chromosome_genes_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(
        genotype.genes_slice(&chromosome),
        &[false, false, true, false, true, true, true, false, false, true]
    );

    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        genotype.genes_slice(&chromosome),
        &[true, false, false, false, true, true, false, false, false, true]
    );
}

#[test]
fn mutate_chromosome_genes_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(
        genotype.genes_slice(&chromosome),
        &[false, false, true, false, true, true, true, false, false, true]
    );

    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert_eq!(
        genotype.genes_slice(&chromosome),
        &[true, true, true, false, true, false, false, false, false, false]
    );
}

#[test]
fn crossover_chromosome_genes() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut father = genotype.chromosome_constructor_random(rng);
    let mut mother = genotype.chromosome_constructor_random(rng);

    // Set father to all true, mother to all false
    // FIXME: don't like this interface
    for i in 0..10 {
        genotype.data[father.row_id][i] = true;
        genotype.data[mother.row_id][i] = false;
    }

    genotype.crossover_chromosome_genes(3, false, &mut father, &mut mother, rng);

    let father_result: Vec<bool> = genotype.genes_slice(&father).to_vec();
    let mother_result: Vec<bool> = genotype.genes_slice(&mother).to_vec();

    // After crossover, exactly 3 genes should have been swapped
    // Count how many genes differ from original (all true for father, all false for mother)
    let father_changes = father_result.iter().filter(|&&x| !x).count();
    let mother_changes = mother_result.iter().filter(|&&x| x).count();

    assert_eq!(father_changes, 3);
    assert_eq!(mother_changes, 3);
}

#[test]
fn crossover_chromosome_points() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut father = genotype.chromosome_constructor_random(rng);
    let mut mother = genotype.chromosome_constructor_random(rng);

    // Set father to all true, mother to all false
    for i in 0..10 {
        genotype.data[father.row_id][i] = true;
        genotype.data[mother.row_id][i] = false;
    }

    genotype.crossover_chromosome_points(2, false, &mut father, &mut mother, rng);

    let father_result: Vec<bool> = genotype.genes_slice(&father).to_vec();
    let mother_result: Vec<bool> = genotype.genes_slice(&mother).to_vec();

    assert_eq!(
        father_result,
        vec![true, true, false, false, false, true, true, true, true, true]
    );
    assert_eq!(
        mother_result,
        vec![false, false, true, true, true, false, false, false, false, false]
    );
}

#[test]
fn neighbouring_population() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<3, 7>::builder()
        .with_genes_size(3)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(genotype.genes_slice(&chromosome), &[false, false, true]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(3u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);

    let result: Vec<Vec<bool>> = population
        .chromosomes
        .iter()
        .map(|c| genotype.genes_slice(c).to_vec())
        .collect();

    assert_eq!(
        result,
        vec![
            vec![true, false, true],
            vec![false, true, true],
            vec![false, false, false]
        ]
    );
}

#[test]
fn chromosome_constructor_with_seed_genes_list() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<4, 4>::builder()
        .with_genes_size(4)
        .with_seed_genes_list(vec![
            Box::new([true, false, true, false]),
            Box::new([false, true, false, true]),
        ])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosomes = [
        genotype.chromosome_constructor_random(&mut rng),
        genotype.chromosome_constructor_random(&mut rng),
        genotype.chromosome_constructor_random(&mut rng),
        genotype.chromosome_constructor_random(&mut rng),
    ];

    assert_eq!(
        genotype.genes_slice(&chromosomes[0]),
        &[false, true, false, true]
    );
    assert_eq!(
        genotype.genes_slice(&chromosomes[1]),
        &[true, false, true, false]
    );
    assert_eq!(
        genotype.genes_slice(&chromosomes[2]),
        &[false, true, false, true]
    );
    assert_eq!(
        genotype.genes_slice(&chromosomes[3]),
        &[true, false, true, false]
    );
}

#[test]
fn population_constructor_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<4, 5>::builder()
        .with_genes_size(4)
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let population = genotype.population_constructor(5, &mut rng);

    let result: Vec<Vec<bool>> = population
        .chromosomes
        .iter()
        .map(|c| genotype.genes_slice(c).to_vec())
        .collect();

    assert_eq!(
        result,
        vec![
            vec![false, false, true, false],
            vec![true, true, true, false],
            vec![false, true, false, true],
            vec![true, false, true, false],
            vec![false, false, true, true],
        ]
    );
}

#[test]
fn population_constructor_with_seed_genes_list() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<4, 5>::builder()
        .with_genes_size(4)
        .with_seed_genes_list(vec![
            Box::new([true, true, false, false]),
            Box::new([false, false, true, true]),
        ])
        .build()
        .unwrap();
    genotype.chromosomes_setup();
    let population = genotype.population_constructor(5, &mut rng);

    let result: Vec<Vec<bool>> = population
        .chromosomes
        .iter()
        .map(|c| genotype.genes_slice(c).to_vec())
        .collect();

    assert_eq!(
        result,
        vec![
            vec![true, true, false, false],
            vec![false, false, true, true],
            vec![true, true, false, false],
            vec![false, false, true, true],
            vec![true, true, false, false],
        ]
    );
}

#[test]
fn chromosome_manager() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<5, 4>::builder()
        .with_genes_size(5)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosomes = (0..4)
        .map(|_| genotype.chromosome_constructor_random(rng))
        .collect::<Vec<_>>();

    genotype.save_best_genes(&chromosomes[2]);

    let results: Vec<Vec<bool>> = chromosomes
        .iter()
        .map(|c| genotype.genes_slice(c).to_vec())
        .collect();

    assert_eq!(
        results,
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
            vec![false, true, true, false, true],
            vec![false, false, false, true, true],
        ]
    );
    assert_eq!(
        genotype.best_genes().as_slice(),
        &[false, true, true, false, true]
    );

    genotype.chromosome_destructor_truncate(&mut chromosomes, 2);

    let results: Vec<Vec<bool>> = chromosomes
        .iter()
        .map(|c| genotype.genes_slice(c).to_vec())
        .collect();

    assert_eq!(
        results,
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
        ]
    );

    genotype.chromosome_cloner_expand(&mut chromosomes, 2);

    let results: Vec<Vec<bool>> = chromosomes
        .iter()
        .map(|c| genotype.genes_slice(c).to_vec())
        .collect();

    assert_eq!(
        results,
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
        ]
    );

    chromosomes
        .iter_mut()
        .take(2)
        .for_each(|c| genotype.mutate_chromosome_genes(3, false, c, None, rng));

    // After mutation, first 2 chromosomes should each have 3 mutations
    // Last 2 should remain unchanged clones
    let results: Vec<Vec<bool>> = chromosomes
        .iter()
        .map(|c| genotype.genes_slice(c).to_vec())
        .collect();

    // Check that the last 2 chromosomes are still clones of the originals
    assert_eq!(results[3], vec![true, true, false, false, true]);
    assert_eq!(results[2], vec![false, false, true, false, true]);

    // Check that best genes were preserved
    assert_eq!(
        genotype.best_genes().as_slice(),
        &[false, true, true, false, true]
    );
}
