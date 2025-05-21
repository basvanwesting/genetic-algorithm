#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::genotype::{
    EvolveGenotype, Genotype, HillClimbGenotype, PermutateGenotype, RangeGenotype,
};

#[test]
fn float_mutate_chromosome_single_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818,],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.976, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818,],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818,],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 1.0, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818,],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 1.0, 0.462, 0.897, 0.942, 0.499, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_scaled_range(vec![-1.0..=1.0, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.969, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.969, 0.462, 0.897, 0.942, 0.598, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn mutate_chromosome_genes_random_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0.0; 10]);
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.0, 0.0, 0.0, 0.818, 0.439, 0.456, 0.0, 0.0, 0.942, 0.462],
        0.001
    ));
}
#[test]
fn mutate_chromosome_genes_random_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0.0; 10]);
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.0, 0.0, 0.395, 0.818, 0.644, 0.0, 0.0, 0.240, 0.976, 0.0],
        0.001
    ));
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
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
    let mut genotype = RangeGenotype::builder()
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
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![vec![0.391], vec![0.545]],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_2_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
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
fn float_neighbouring_population_2_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
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
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_scaled_range(vec![-0.5..=0.5, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(0), &mut rng);
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
    genotype.chromosome_destructor_truncate(&mut population.chromosomes, 0);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(1), &mut rng);
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
    genotype.chromosome_destructor_truncate(&mut population.chromosomes, 0);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(2), &mut rng);
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
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(-0.1..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.980],
        0.001,
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.394, 0.439, 0.980],
            vec![0.537, 0.439, 0.980],
            vec![0.447, 0.433, 0.980],
            vec![0.447, 0.498, 0.980],
            vec![0.447, 0.439, 0.925],
            vec![0.447, 0.439, 0.987],
        ],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_3_one_sided() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_allele_mutation_range(0.0..=0.1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.980],
        0.001,
    ));

    // size makes error as it counts 0.0 twice, this is fine
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.494, 0.439, 0.980],
            vec![0.447, 0.529, 0.980],
            vec![0.447, 0.439, 0.999],
        ],
        0.001,
    ));
}

#[test]
fn float_allele_values_scaled() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=0.55)
        .with_allele_mutation_scaled_range(vec![-0.5..=0.5, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();

    assert!(relative_chromosome_eq(
        genotype.allele_values_scaled(1),
        vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.55],
        0.001
    ));
}

#[test]
fn float_chromosome_permutations_2_scaled() {
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=0.25)
        .with_allele_mutation_scaled_range(vec![-0.5..=0.5, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    assert_eq!(
        genotype.chromosome_permutations_size(Some(1)),
        BigUint::from(16u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(1))
        .collect::<Vec<_>>();

    assert!(relative_population_eq(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0.0, 0.0],
            vec![0.0, 0.1],
            vec![0.0, 0.2],
            vec![0.0, 0.25],
            vec![0.1, 0.0],
            vec![0.1, 0.1],
            vec![0.1, 0.2],
            vec![0.1, 0.25],
            vec![0.2, 0.0],
            vec![0.2, 0.1],
            vec![0.2, 0.2],
            vec![0.2, 0.25],
            vec![0.25, 0.0],
            vec![0.25, 0.1],
            vec![0.25, 0.2],
            vec![0.25, 0.25],
        ],
        0.001,
    ));
}

#[test]
fn integer_mutate_chromosome_single_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 5, 4, 3, 8],
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 0, 4, 3, 8],
    );
}

#[test]
fn integer_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .with_allele_mutation_range(-1..=1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 5, 4, 3, 8],
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 4, 4, 3, 8],
    );
}

#[test]
fn integer_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0..=9)
        .with_allele_mutation_range(-1..=1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(inspect::population(&population), vec![vec![3], vec![5]],);
}

#[test]
fn integer_neighbouring_population_2_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![2, 4], vec![7, 4], vec![4, 3], vec![4, 6]]
    );
}

#[test]
fn integer_neighbouring_population_2_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_allele_mutation_range(-2..=2)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![3, 4], vec![5, 4], vec![4, 3], vec![4, 5]]
    );
}

#[test]
fn integer_neighbouring_population_2_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_allele_mutation_scaled_range(vec![-3..=3, -2..=2, -1..=1])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(1), &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![2, 4], vec![6, 4], vec![4, 2], vec![4, 6]]
    );
}

#[test]
fn integer_neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0..=9)
        .with_allele_mutation_range(-1..=1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 9]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
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
fn integer_neighbouring_population_3_one_sided() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0..=9)
        .with_allele_mutation_range(0..=1)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 9]);

    // size makes error as it counts 0.0 twice, this is fine
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));

    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![5, 4, 9], vec![4, 5, 9]]
    );
}

#[test]
fn integer_allele_values_scaled() {
    let genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_allele_mutation_scaled_range(vec![-3..=3, -2..=2, -1..=1])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_values_scaled(1), vec![0, 2, 4, 6, 8, 9]);
}

#[test]
fn integer_chromosome_permutations_2_scaled() {
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=5)
        .with_allele_mutation_scaled_range(vec![-3..=3, -2..=2, -1..=1])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    assert_eq!(
        genotype.chromosome_permutations_size(Some(1)),
        BigUint::from(16u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(1))
        .collect::<Vec<_>>();

    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0, 0],
            vec![0, 2],
            vec![0, 4],
            vec![0, 5],
            vec![2, 0],
            vec![2, 2],
            vec![2, 4],
            vec![2, 5],
            vec![4, 0],
            vec![4, 2],
            vec![4, 4],
            vec![4, 5],
            vec![5, 0],
            vec![5, 2],
            vec![5, 4],
            vec![5, 5],
        ]
    );

    // assert!(relative_population_eq(
    //     inspect::chromosomes(&chromosomes),
    //     vec![vec![0.391], vec![0.545]],
    //     0.001,
    // ));
}

#[test]
fn float_calculate_genes_hash() {
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_genes_hashing(true)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome_1 = build::chromosome(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);
    let chromosome_2 = build::chromosome(vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);
    let chromosome_3 = build::chromosome(vec![-0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);
    let chromosome_4 = build::chromosome(vec![-0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]);

    assert!(genotype.calculate_genes_hash(&chromosome_1).is_some());
    // assert_eq!(
    //     genotype.calculate_genes_hash(&chromosome_1),
    //     Some(13948481349068670127)
    // );
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_1),
        genotype.calculate_genes_hash(&chromosome_2),
    );
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_3),
        genotype.calculate_genes_hash(&chromosome_4),
    );

    // the sign on zero matters
    assert_ne!(
        genotype.calculate_genes_hash(&chromosome_1),
        genotype.calculate_genes_hash(&chromosome_3),
    );
}

#[test]
fn integer_calculate_genes_hash() {
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(-10..=10)
        .with_genes_hashing(true)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome_1 = build::chromosome(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_2 = build::chromosome(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_3 = build::chromosome(vec![-0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let chromosome_4 = build::chromosome(vec![-0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert!(genotype.calculate_genes_hash(&chromosome_1).is_some());
    // assert_eq!(
    //     genotype.calculate_genes_hash(&chromosome_1),
    //     Some(10064628735429642131)
    // );
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_1),
        genotype.calculate_genes_hash(&chromosome_2),
    );
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_3),
        genotype.calculate_genes_hash(&chromosome_4),
    );

    // the sign on does not matter
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_1),
        genotype.calculate_genes_hash(&chromosome_3),
    );
}
