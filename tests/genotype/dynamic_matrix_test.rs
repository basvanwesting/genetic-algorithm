#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::genotype::{DynamicMatrixGenotype, Genotype, IncrementalGenotype};

#[test]
fn chromosome_constructor() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.976, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 1.0, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 1.0, 0.462, 0.897, 0.942, 0.499, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_scaled_range(vec![-1.0..=1.0, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.969, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.969, 0.462, 0.897, 0.942, 0.598, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn mutate_chromosome_genes_random_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.296, 0.462, 0.897, 0.942, 0.054, 0.724, 0.395, 0.225],
        0.001,
    ));
}
#[test]
fn mutate_chromosome_genes_random_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.787, 0.225, 0.979, 0.462, 0.897, 0.296, 0.232, 0.456, 0.395, 0.724],
        0.001,
    ));
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut father = genotype.chromosome_constructor(rng);
    let mut mother = genotype.chromosome_constructor(rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&father).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(&mother).to_vec(),
        vec![0.240, 0.976, 0.644, 0.054, 0.921, 0.225, 0.232, 0.296, 0.787, 0.724],
        0.001
    ));
    genotype.crossover_chromosome_genes(3, false, &mut father, &mut mother, rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&father).to_vec(),
        vec![0.447, 0.976, 0.644, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.724],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(&mother).to_vec(),
        vec![0.240, 0.439, 0.979, 0.054, 0.921, 0.225, 0.232, 0.296, 0.787, 0.818],
        0.001
    ));
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut father = genotype.chromosome_constructor(rng);
    let mut mother = genotype.chromosome_constructor(rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&father).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(&mother).to_vec(),
        vec![0.240, 0.976, 0.644, 0.054, 0.921, 0.225, 0.232, 0.296, 0.787, 0.724],
        0.001
    ));
    genotype.crossover_chromosome_points(2, false, &mut father, &mut mother, rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&father).to_vec(),
        vec![0.447, 0.439, 0.644, 0.054, 0.921, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(&mother).to_vec(),
        vec![0.240, 0.976, 0.979, 0.462, 0.897, 0.225, 0.232, 0.296, 0.787, 0.724],
        0.001
    ));
}

#[test]
fn float_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447],
        0.001
    ));

    let population = genotype.neighbouring_population(&chromosome, None, &mut rng);
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![vec![0.391], vec![0.545]],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_2_unscaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439],
        0.001
    ));

    let population = genotype.neighbouring_population(&chromosome, None, &mut rng);
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.445, 0.439],
            vec![0.494, 0.439],
            vec![0.447, 0.429],
            vec![0.447, 0.533],
        ],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_2_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_scaled_range(vec![-0.5..=0.5, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));

    let population = genotype.neighbouring_population(&chromosome, Some(0), &mut rng);
    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.0, 0.439],
            vec![0.947, 0.439],
            vec![0.447, 0.0],
            vec![0.447, 0.939],
        ],
        0.001,
    ));

    let population = genotype.neighbouring_population(&chromosome, Some(1), &mut rng);
    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.347, 0.439],
            vec![0.547, 0.439],
            vec![0.447, 0.339],
            vec![0.447, 0.539],
        ],
        0.001,
    ));

    let population = genotype.neighbouring_population(&chromosome, Some(2), &mut rng);
    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.437, 0.439],
            vec![0.457, 0.439],
            vec![0.447, 0.429],
            vec![0.447, 0.449],
        ],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_3_one_sided() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(0.0..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let chromosome = genotype.chromosome_constructor(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(&chromosome).to_vec(),
        vec![0.447, 0.439, 0.980],
        0.001
    ));

    // size makes error as it counts 0.0 twice, this is fine
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));

    let population = genotype.neighbouring_population(&chromosome, None, &mut rng);
    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.494, 0.439, 0.980],
            vec![0.447, 0.529, 0.980],
            vec![0.447, 0.439, 0.999],
        ],
        0.001,
    ));
}

#[test]
fn chromosome_manager() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = DynamicMatrixGenotype::builder()
        .with_genes_size(5)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    genotype.chromosomes_init();

    let mut chromosomes = (0..4)
        .map(|_| genotype.chromosome_constructor(rng))
        .collect::<Vec<_>>();
    genotype.save_best_genes(&chromosomes[2]);
    dbg!("init", &chromosomes, &genotype.best_genes());

    assert!(relative_population_eq(
        chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
            vec![0.240, 0.976, 0.644, 0.054, 0.921],
            vec![0.225, 0.232, 0.296, 0.787, 0.724],
        ],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.best_genes().to_vec(),
        vec![0.240, 0.976, 0.644, 0.054, 0.921],
        0.001
    ));

    genotype.chromosome_destructor_truncate(&mut chromosomes, 2);
    dbg!("truncate", &chromosomes, &genotype.best_genes());

    assert!(relative_population_eq(
        chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
        ],
        0.001
    ));

    genotype.chromosome_cloner_range(&mut chromosomes, 0..2);
    dbg!("clone range", &chromosomes, &genotype.best_genes());

    assert!(relative_population_eq(
        chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
        ],
        0.001
    ));

    chromosomes
        .iter_mut()
        .take(2)
        .for_each(|c| genotype.mutate_chromosome_genes(3, false, c, None, rng));
    dbg!("mutate", &chromosomes, &genotype.best_genes());

    assert!(relative_population_eq(
        chromosomes
            .iter()
            .map(|c| genotype.get_genes(c).to_vec())
            .collect(),
        vec![
            vec![0.447, 0.900, 0.979, 0.390, 0.971],
            vec![0.848, 0.588, 0.346, 0.014, 0.818],
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
        ],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.best_genes().to_vec(),
        vec![0.240, 0.976, 0.644, 0.054, 0.921],
        0.001
    ));
}
