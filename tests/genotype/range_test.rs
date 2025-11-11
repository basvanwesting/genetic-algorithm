#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    EvolveGenotype, Genotype, HillClimbGenotype, MutationType, PermutateGenotype, RangeGenotype,
};

#[test]
fn sample_gene_indices() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    assert_eq!(
        genotype.sample_gene_indices(10, false, &mut rng),
        vec![5, 0, 8, 9, 7, 2, 4, 1, 3, 6]
    );
    assert_eq!(
        genotype.sample_gene_indices(10, true, &mut rng),
        vec![5, 1, 2, 8, 3, 9, 9, 0, 8, 4]
    );
}
#[test]
fn float_mutate_chromosome_single_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818,],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.976, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818,],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_range() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.528, 0.979],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.483, 0.979],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_range_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::RangeScaled(vec![1.0, 0.1, 0.01]))
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979],
        0.001,
    ));

    assert_eq!(genotype.current_scale_index, 0);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.897, 0.979],
        0.001,
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.921, 0.989],
        0.001,
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.921, 0.982],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_step() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Step(0.1))
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.539, 0.979],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.539, 1.0],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_step_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::StepScaled(vec![1.0, 0.1, 0.01]))
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979],
        0.001,
    ));

    assert_eq!(genotype.current_scale_index, 0);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 1.0, 0.979],
        0.001,
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.9, 1.0],
        0.001,
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.9, 0.99],
        0.001,
    ));
}

#[test]
fn mutate_chromosome_genes_random_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0.0; 10]);
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.0, 0.0, 0.0, 0.818, 0.439, 0.456, 0.0, 0.0, 0.942, 0.462],
        0.001
    ));
}
#[test]
fn mutate_chromosome_genes_random_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0.0; 10]);
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.0, 0.0, 0.395, 0.818, 0.644, 0.0, 0.0, 0.240, 0.976, 0.0],
        0.001
    ));
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=2.0)
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);
    let mut mother = build::chromosome(vec![1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]);
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![0.0, 0.1, 0.2, 0.3, 1.4, 0.5, 0.6, 0.7, 0.8, 0.9]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![1.0, 1.1, 1.2, 1.3, 0.4, 1.5, 1.6, 1.7, 1.8, 1.9]
    );
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);
    let mut mother = build::chromosome(vec![1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![0.0, 0.1, 0.2, 0.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![1.0, 1.1, 1.2, 1.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]
    );
}

#[test]
fn float_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![vec![0.403], vec![0.491]],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_2_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.438, 0.439],
            vec![0.702, 0.439],
            vec![0.447, 0.393],
            vec![0.447, 0.968],
        ],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_2_range() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.349, 0.439],
            vec![0.545, 0.439],
            vec![0.447, 0.392],
            vec![0.447, 0.485],
        ],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_2_step_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::StepScaled(vec![0.5, 0.1, 0.01]))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.0, 0.439],
            vec![0.947, 0.439],
            vec![0.447, 0.0],
            vec![0.447, 0.939],
        ],
        0.001,
    ));
    population.chromosomes.truncate(0);
    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.347, 0.439],
            vec![0.547, 0.439],
            vec![0.447, 0.339],
            vec![0.447, 0.539],
        ],
        0.001,
    ));
    population.chromosomes.truncate(0);
    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
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
fn float_neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Range(0.1))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.980],
        0.001,
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.401, 0.439, 0.979],
            vec![0.493, 0.439, 0.979],
            vec![0.447, 0.349, 0.979],
            vec![0.447, 0.528, 0.979],
            vec![0.447, 0.439, 0.885],
            vec![0.447, 0.439, 1.0],
        ],
        0.001,
    ));
}

#[test]
fn float_permutable_gene_values_step_scaled() {
    let scaled_steps = &vec![1.0, 0.1, 0.01];
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=10.0)
        .with_mutation_type(MutationType::StepScaled(scaled_steps.clone()))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![4.473, 4.391],
        0.001,
    ));

    assert!(relative_chromosome_eq(
        genotype.permutable_gene_values_step_scaled(0, Some(&chromosome), scaled_steps),
        vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        0.001
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    assert!(relative_chromosome_eq(
        genotype.permutable_gene_values_step_scaled(0, Some(&chromosome), scaled_steps),
        vec![
            3.473, 3.573, 3.673, 3.773, 3.873, 3.973, 4.073, 4.173, 4.273, 4.373, 4.473, 4.573,
            4.673, 4.773, 4.873, 4.973, 5.073, 5.173, 5.273, 5.373, 5.473, 5.473,
        ],
        0.001
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    assert!(relative_chromosome_eq(
        genotype.permutable_gene_values_step_scaled(0, Some(&chromosome), scaled_steps),
        vec![
            4.373, 4.383, 4.393, 4.403, 4.413, 4.423, 4.433, 4.443, 4.453, 4.463, 4.473, 4.483,
            4.493, 4.503, 4.513, 4.523, 4.533, 4.543, 4.553, 4.563, 4.573, 4.573,
        ],
        0.001
    ));
}

#[test]
fn float_chromosome_permutations_2_step_scaled() {
    let scaled_steps = &vec![5.0, 2.0, 1.0];
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=10.0)
        .with_mutation_type(MutationType::StepScaled(scaled_steps.clone()))
        .build()
        .unwrap();

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(70u32)
    );

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![4.473, 4.391],
        0.001
    ));

    assert_eq!(genotype.current_scale_index, 0);
    assert_eq!(
        genotype.chromosome_permutations_size_for_scale_index(0),
        BigUint::from(9u32)
    );

    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome))
        .collect::<Vec<_>>();

    assert!(relative_population_eq(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0.0, 0.0],
            vec![0.0, 5.0],
            vec![0.0, 10.0],
            vec![5.0, 0.0],
            vec![5.0, 5.0],
            vec![5.0, 10.0],
            vec![10.0, 0.0],
            vec![10.0, 5.0],
            vec![10.0, 10.0]
        ],
        0.001,
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    assert_eq!(
        genotype.chromosome_permutations_size_for_scale_index(1),
        BigUint::from(36u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome))
        .collect::<Vec<_>>();

    assert!(relative_population_eq(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0.0, 0.0],
            vec![0.0, 2.0],
            vec![0.0, 4.0],
            vec![0.0, 6.0],
            vec![0.0, 8.0],
            vec![0.0, 9.3],
            vec![2.0, 0.0],
            vec![2.0, 2.0],
            vec![2.0, 4.0],
            vec![2.0, 6.0],
            vec![2.0, 8.0],
            vec![2.0, 9.3],
            vec![4.0, 0.0],
            vec![4.0, 2.0],
            vec![4.0, 4.0],
            vec![4.0, 6.0],
            vec![4.0, 8.0],
            vec![4.0, 9.3],
            vec![6.0, 0.0],
            vec![6.0, 2.0],
            vec![6.0, 4.0],
            vec![6.0, 6.0],
            vec![6.0, 8.0],
            vec![6.0, 9.3],
            vec![8.0, 0.0],
            vec![8.0, 2.0],
            vec![8.0, 4.0],
            vec![8.0, 6.0],
            vec![8.0, 8.0],
            vec![8.0, 9.3],
            vec![9.4, 0.0],
            vec![9.4, 2.0],
            vec![9.4, 4.0],
            vec![9.4, 6.0],
            vec![9.4, 8.0],
            vec![9.4, 9.3],
        ],
        0.1,
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    assert_eq!(
        genotype.chromosome_permutations_size_for_scale_index(2),
        BigUint::from(25u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome))
        .collect::<Vec<_>>();

    assert!(relative_population_eq(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![2.473, 2.391],
            vec![2.473, 3.391],
            vec![2.473, 4.391],
            vec![2.473, 5.391],
            vec![2.473, 6.391],
            vec![3.473, 2.391],
            vec![3.473, 3.391],
            vec![3.473, 4.391],
            vec![3.473, 5.391],
            vec![3.473, 6.391],
            vec![4.473, 2.391],
            vec![4.473, 3.391],
            vec![4.473, 4.391],
            vec![4.473, 5.391],
            vec![4.473, 6.391],
            vec![5.473, 2.391],
            vec![5.473, 3.391],
            vec![5.473, 4.391],
            vec![5.473, 5.391],
            vec![5.473, 6.391],
            vec![6.473, 2.391],
            vec![6.473, 3.391],
            vec![6.473, 4.391],
            vec![6.473, 5.391],
            vec![6.473, 6.391],
        ],
        0.001,
    ));
}

#[test]
fn integer_mutate_chromosome_single_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 5, 4, 3, 8],
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 0, 4, 3, 8],
    );
}

#[test]
fn integer_mutate_chromosome_single_range() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Range(1))
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 5, 4, 3, 8],
    );

    genotype.mutate_chromosome_genes(5, true, &mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![3, 4, 8, 4, 8, 9, 5, 3, 3, 8],
    );
}

#[test]
fn integer_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Range(1))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(inspect::population(&population), vec![vec![3], vec![5]],);
}

#[test]
fn integer_neighbouring_population_2_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![2, 4], vec![7, 4], vec![4, 3], vec![4, 6]]
    );
}

#[test]
fn integer_neighbouring_population_2_range() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Range(2))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![2, 4], vec![6, 4], vec![4, 3], vec![4, 5]]
    );
}

#[test]
fn integer_neighbouring_population_2_step_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::StepScaled(vec![3, 2, 1]))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![], true);
    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![2, 4], vec![6, 4], vec![4, 2], vec![4, 6]]
    );
}

#[test]
fn integer_neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Range(1))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 9]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![3, 4, 9],
            vec![5, 4, 9],
            vec![4, 3, 9],
            vec![4, 5, 9],
            vec![4, 4, 8],
        ]
    );
}

#[test]
fn integer_permutable_gene_values_step_scaled() {
    let scaled_steps = &vec![100, 10, 1];
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=1000)
        .with_mutation_type(MutationType::StepScaled(scaled_steps.clone()))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![447, 439]);

    assert_eq!(genotype.current_scale_index, 0);
    assert_eq!(
        genotype.permutable_gene_values_step_scaled(0, Some(&chromosome), scaled_steps),
        vec![0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000],
    );
    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    assert_eq!(
        genotype.permutable_gene_values_step_scaled(0, Some(&chromosome), scaled_steps),
        vec![
            347, 357, 367, 377, 387, 397, 407, 417, 427, 437, 447, 457, 467, 477, 487, 497, 507,
            517, 527, 537, 547
        ],
    );
    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    assert_eq!(
        genotype.permutable_gene_values_step_scaled(0, Some(&chromosome), scaled_steps),
        vec![
            437, 438, 439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453,
            454, 455, 456, 457
        ],
    );
}

#[test]
fn integer_chromosome_permutations_2_step_scaled() {
    let scaled_steps = &vec![5, 2, 1];
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=10)
        .with_mutation_type(MutationType::StepScaled(scaled_steps.clone()))
        .build()
        .unwrap();

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(70u32)
    );

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4]);

    assert_eq!(genotype.current_scale_index, 0);
    assert_eq!(
        genotype.chromosome_permutations_size_for_scale_index(0),
        BigUint::from(9u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome))
        .collect::<Vec<_>>();
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0, 0],
            vec![0, 5],
            vec![0, 10],
            vec![5, 0],
            vec![5, 5],
            vec![5, 10],
            vec![10, 0],
            vec![10, 5],
            vec![10, 10],
        ]
    );

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    assert_eq!(
        genotype.chromosome_permutations_size_for_scale_index(1),
        BigUint::from(36u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome))
        .collect::<Vec<_>>();
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0, 0],
            vec![0, 2],
            vec![0, 4],
            vec![0, 6],
            vec![0, 8],
            vec![0, 9],
            vec![2, 0],
            vec![2, 2],
            vec![2, 4],
            vec![2, 6],
            vec![2, 8],
            vec![2, 9],
            vec![4, 0],
            vec![4, 2],
            vec![4, 4],
            vec![4, 6],
            vec![4, 8],
            vec![4, 9],
            vec![6, 0],
            vec![6, 2],
            vec![6, 4],
            vec![6, 6],
            vec![6, 8],
            vec![6, 9],
            vec![8, 0],
            vec![8, 2],
            vec![8, 4],
            vec![8, 6],
            vec![8, 8],
            vec![8, 9],
            vec![9, 0],
            vec![9, 2],
            vec![9, 4],
            vec![9, 6],
            vec![9, 8],
            vec![9, 9],
        ]
    );

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    assert_eq!(
        genotype.chromosome_permutations_size_for_scale_index(2),
        BigUint::from(25u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome))
        .collect::<Vec<_>>();
    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![2, 2],
            vec![2, 3],
            vec![2, 4],
            vec![2, 5],
            vec![2, 6],
            vec![3, 2],
            vec![3, 3],
            vec![3, 4],
            vec![3, 5],
            vec![3, 6],
            vec![4, 2],
            vec![4, 3],
            vec![4, 4],
            vec![4, 5],
            vec![4, 6],
            vec![5, 2],
            vec![5, 3],
            vec![5, 4],
            vec![5, 5],
            vec![5, 6],
            vec![6, 2],
            vec![6, 3],
            vec![6, 4],
            vec![6, 5],
            vec![6, 6],
        ]
    );
}

#[test]
fn float_calculate_genes_hash() {
    let chromosome_1: Chromosome<f32> = build::chromosome_without_genes_hash(vec![
        0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,
    ]);
    let chromosome_2: Chromosome<f32> = build::chromosome_without_genes_hash(vec![
        0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,
    ]);
    let chromosome_3: Chromosome<f32> = build::chromosome_without_genes_hash(vec![
        -0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,
    ]);
    let chromosome_4: Chromosome<f32> = build::chromosome_without_genes_hash(vec![
        -0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9,
    ]);

    let hash_1 = chromosome_1.calculate_hash();
    let hash_2 = chromosome_2.calculate_hash();
    let hash_3 = chromosome_3.calculate_hash();
    let hash_4 = chromosome_4.calculate_hash();

    assert_ne!(hash_1, 0);

    // Same genes should have same hash
    assert_eq!(hash_1, hash_2);
    assert_eq!(hash_3, hash_4);

    // the sign on zero matters
    assert_ne!(hash_1, hash_3);
}

#[test]
fn integer_calculate_genes_hash() {
    let chromosome_1: Chromosome<i32> =
        build::chromosome_without_genes_hash(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_2: Chromosome<i32> =
        build::chromosome_without_genes_hash(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_3: Chromosome<i32> =
        build::chromosome_without_genes_hash(vec![-0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_4: Chromosome<i32> =
        build::chromosome_without_genes_hash(vec![-0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let hash_1 = chromosome_1.calculate_hash();
    let hash_2 = chromosome_2.calculate_hash();
    let hash_3 = chromosome_3.calculate_hash();
    let hash_4 = chromosome_4.calculate_hash();

    assert_ne!(hash_1, 0);

    // Same genes should have same hash
    assert_eq!(hash_1, hash_2);
    assert_eq!(hash_3, hash_4);

    // the sign on does not matter
    assert_eq!(hash_1, hash_3);
}
