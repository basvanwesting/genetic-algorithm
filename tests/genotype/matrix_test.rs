#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{Genotype, MatrixGenotype};

#[test]
fn chromosome_factory() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MatrixGenotype::<f32, 10, 5>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.976, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MatrixGenotype::<f32, 10, 5>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 1.0, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 1.0, 0.462, 0.897, 0.942, 0.499, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MatrixGenotype::<f32, 10, 5>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_scaled_range(vec![-1.0..=1.0, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.969, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.969, 0.462, 0.897, 0.942, 0.598, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn mutate_chromosome_genes_random_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MatrixGenotype::<f32, 10, 5>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.296, 0.462, 0.897, 0.942, 0.054, 0.724, 0.395, 0.225],
        0.001,
    ));
}
#[test]
fn mutate_chromosome_genes_random_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MatrixGenotype::<f32, 10, 5>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(chromosome.reference_id).to_vec(),
        vec![0.787, 0.225, 0.979, 0.462, 0.897, 0.296, 0.232, 0.456, 0.395, 0.724],
        0.001,
    ));
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MatrixGenotype::<f32, 10, 5>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut father = genotype.chromosome_factory(rng);
    let mut mother = genotype.chromosome_factory(rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(father.reference_id).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(mother.reference_id).to_vec(),
        vec![0.240, 0.976, 0.644, 0.054, 0.921, 0.225, 0.232, 0.296, 0.787, 0.724],
        0.001
    ));
    genotype.crossover_chromosome_genes(3, false, &mut father, &mut mother, rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(father.reference_id).to_vec(),
        vec![0.447, 0.976, 0.644, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.724],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(mother.reference_id).to_vec(),
        vec![0.240, 0.439, 0.979, 0.054, 0.921, 0.225, 0.232, 0.296, 0.787, 0.818],
        0.001
    ));
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MatrixGenotype::<f32, 10, 5>::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut father = genotype.chromosome_factory(rng);
    let mut mother = genotype.chromosome_factory(rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(father.reference_id).to_vec(),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(mother.reference_id).to_vec(),
        vec![0.240, 0.976, 0.644, 0.054, 0.921, 0.225, 0.232, 0.296, 0.787, 0.724],
        0.001
    ));
    genotype.crossover_chromosome_points(2, false, &mut father, &mut mother, rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(father.reference_id).to_vec(),
        vec![0.447, 0.439, 0.644, 0.054, 0.921, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(mother.reference_id).to_vec(),
        vec![0.240, 0.976, 0.979, 0.462, 0.897, 0.225, 0.232, 0.296, 0.787, 0.724],
        0.001
    ));
}

#[test]
fn population_sync() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MatrixGenotype::<f32, 5, 5>::builder()
        .with_genes_size(5)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut population = Population::new(
        (0..4)
            .map(|_| genotype.chromosome_factory(rng))
            .collect::<Vec<_>>(),
    );
    let best_chromosome = genotype.chromosome_factory(rng);
    assert!(relative_chromosome_eq(
        genotype.get_genes(best_chromosome.reference_id).to_vec(),
        vec![0.582, 0.186, 0.253, 0.864, 0.390],
        0.001
    ));

    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c.reference_id).to_vec())
            .collect(),
        vec![
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
            vec![0.240, 0.976, 0.644, 0.054, 0.921],
            vec![0.225, 0.232, 0.296, 0.787, 0.724],
        ],
        0.001
    ));

    population.truncate(2);
    genotype.population_sync(&mut population, &best_chromosome);

    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c.reference_id).to_vec())
            .collect(),
        vec![
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
        ],
        0.001
    ));

    population.chromosomes.extend_from_within(..);
    genotype.population_sync(&mut population, &best_chromosome);

    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c.reference_id).to_vec())
            .collect(),
        vec![
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
        ],
        0.001
    ));

    population
        .chromosomes
        .iter_mut()
        .take(2)
        .for_each(|c| genotype.mutate_chromosome_genes(3, false, c, None, rng));

    assert!(relative_population_eq(
        population
            .chromosomes
            .iter()
            .map(|c| genotype.get_genes(c.reference_id).to_vec())
            .collect(),
        vec![
            vec![0.848, 0.439, 0.435, 0.462, 0.014],
            vec![0.942, 0.021, 0.456, 0.687, 0.409],
            vec![0.447, 0.439, 0.979, 0.462, 0.897],
            vec![0.942, 0.588, 0.456, 0.395, 0.818],
        ],
        0.001
    ));
    assert!(relative_chromosome_eq(
        genotype.get_genes(best_chromosome.reference_id).to_vec(),
        vec![0.582, 0.186, 0.253, 0.864, 0.390],
        0.001
    ));
}
