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
fn float_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Relative(-0.1..=0.1))
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
        vec![0.447, 0.439, 1.0, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818,],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 1.0, 0.462, 0.897, 0.942, 0.499, 0.456, 0.395, 0.818],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_transition() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=10.0)
        .with_mutation_type(MutationType::Transition(10, 100, -0.1..=0.1))
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![4.473, 4.391, 9.798],
        0.001,
    ));

    // full random
    (0..5).for_each(|_| genotype.increment_generation());
    genotype.mutate_chromosome_genes(3, false, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![3.951, 2.409, 8.188],
        0.001,
    ));
    genotype.mutate_chromosome_genes(3, false, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![7.242, 2.969, 7.878],
        0.001,
    ));

    // 75% transition to relative
    (0..70).for_each(|_| genotype.increment_generation());
    genotype.mutate_chromosome_genes(3, false, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![7.638, 3.345, 5.659],
        0.001,
    ));
    genotype.mutate_chromosome_genes(3, false, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![5.507, 4.640, 5.974],
        0.001,
    ));

    // full relative
    (0..30).for_each(|_| genotype.increment_generation());
    genotype.mutate_chromosome_genes(3, false, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![5.472, 4.611, 6.013],
        0.001,
    ));
    genotype.mutate_chromosome_genes(3, false, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![5.558, 4.549, 5.932],
        0.001,
    ));
}

#[test]
fn float_mutate_chromosome_single_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Scaled(vec![
            -1.0..=1.0,
            -0.1..=0.1,
            -0.01..=0.01,
        ]))
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(genotype.current_scale_index, 0);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.979, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    assert!(genotype.increment_scale_index());
    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.969, 0.462, 0.897, 0.942, 0.588, 0.456, 0.395, 0.818],
        0.001,
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.969, 0.462, 0.897, 0.942, 0.598, 0.456, 0.395, 0.818],
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
        .with_mutation_type(MutationType::Relative(-0.1..=0.1))
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
        vec![vec![0.391], vec![0.545]],
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
fn float_neighbouring_population_2_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Relative(-0.1..=0.1))
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
            vec![0.445, 0.439],
            vec![0.494, 0.439],
            vec![0.447, 0.429],
            vec![0.447, 0.533],
        ],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_2_transition() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=10.0)
        .with_mutation_type(MutationType::Transition(10, 100, -0.1..=0.1))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![4.4732, 4.391],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));

    // full random
    (0..5).for_each(|_| genotype.increment_generation());
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![4.383, 4.391],
            vec![7.027, 4.391],
            vec![4.473, 3.939],
            vec![4.473, 9.680],
        ],
        0.001,
    ));

    // 75% transition to relative
    (0..70).for_each(|_| genotype.increment_generation());
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![3.931, 4.391],
            vec![5.206, 4.391],
            vec![4.473, 3.609],
            vec![4.473, 5.726],
        ],
        0.001,
    ));

    // full relative
    (0..30).for_each(|_| genotype.increment_generation());
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![4.397, 4.391],
            vec![4.570, 4.391],
            vec![4.473, 4.355],
            vec![4.473, 4.396],
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
        .with_mutation_type(MutationType::Scaled(vec![
            -0.5..=0.5,
            -0.1..=0.1,
            -0.01..=0.01,
        ]))
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
        .with_mutation_type(MutationType::Relative(-0.1..=0.1))
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
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::Relative(0.0..=0.1))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.980],
        0.001,
    ));

    // size makes error as it counts 0.0 twice, this is fine
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
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
fn float_permutable_gene_values_scaled() {
    let scaled_ranges = &vec![-1.0..=1.0, -0.1..=0.1, -0.01..=0.01];
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=10.0)
        .with_mutation_type(MutationType::Scaled(scaled_ranges.clone()))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![4.473, 4.391],
        0.001,
    ));

    assert!(relative_population_eq(
        genotype.permutable_gene_values_scaled(Some(&chromosome), scaled_ranges),
        vec![
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        ],
        0.001
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    assert!(relative_population_eq(
        genotype.permutable_gene_values_scaled(Some(&chromosome), scaled_ranges),
        vec![
            vec![
                3.473, 3.573, 3.673, 3.773, 3.873, 3.973, 4.073, 4.173, 4.273, 4.373, 4.473, 4.573,
                4.673, 4.773, 4.873, 4.973, 5.073, 5.173, 5.273, 5.373, 5.473, 5.473,
            ],
            vec![
                3.391, 3.491, 3.591, 3.691, 3.791, 3.891, 3.991, 4.091, 4.191, 4.291, 4.391, 4.491,
                4.591, 4.691, 4.791, 4.891, 4.991, 5.091, 5.191, 5.291, 5.391, 5.391,
            ]
        ],
        0.001
    ));

    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    assert!(relative_population_eq(
        genotype.permutable_gene_values_scaled(Some(&chromosome), scaled_ranges),
        vec![
            vec![
                4.373, 4.383, 4.393, 4.403, 4.413, 4.423, 4.433, 4.443, 4.453, 4.463, 4.473, 4.483,
                4.493, 4.503, 4.513, 4.523, 4.533, 4.543, 4.553, 4.563, 4.573, 4.573,
            ],
            vec![
                4.291, 4.301, 4.311, 4.321, 4.331, 4.341, 4.351, 4.361, 4.371, 4.381, 4.391, 4.401,
                4.411, 4.421, 4.431, 4.441, 4.451, 4.461, 4.471, 4.481, 4.491, 4.491,
            ]
        ],
        0.001
    ));
}

#[test]
fn float_chromosome_permutations_2_scaled() {
    let scaled_ranges = &vec![-5.0..=5.0, -2.0..=2.0, -1.0..=1.0];
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=10.0)
        .with_mutation_type(MutationType::Scaled(scaled_ranges.clone()))
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
        genotype.chromosome_permutations_size_scaled(0, scaled_ranges),
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
        genotype.chromosome_permutations_size_scaled(1, scaled_ranges),
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
        genotype.chromosome_permutations_size_scaled(2, scaled_ranges),
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
fn integer_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Relative(-1..=1))
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
        vec![4, 4, 9, 4, 8, 9, 4, 4, 3, 8],
    );
}

#[test]
fn integer_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Relative(-1..=1))
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
fn integer_neighbouring_population_2_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Relative(-2..=2))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![3, 4], vec![5, 4], vec![4, 3], vec![4, 5]]
    );
}

#[test]
fn integer_neighbouring_population_2_transition() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=100)
        .with_mutation_type(MutationType::Transition(10, 100, -2..=2))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![45, 44],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));

    // full random
    (0..5).for_each(|_| genotype.increment_generation());
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![44, 44], vec![71, 44], vec![45, 39], vec![45, 97]]
    );

    // 75% transition to relative
    (0..70).for_each(|_| genotype.increment_generation());
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![39, 44], vec![50, 44], vec![45, 43], vec![45, 50]]
    );

    // full relative
    (0..30).for_each(|_| genotype.increment_generation());
    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![44, 44], vec![47, 44], vec![45, 42], vec![45, 45]]
    );
}

#[test]
fn integer_neighbouring_population_2_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Scaled(vec![-3..=3, -2..=2, -1..=1]))
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
        .with_mutation_type(MutationType::Relative(-1..=1))
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
fn integer_neighbouring_population_3_one_sided() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = RangeGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0..=9)
        .with_mutation_type(MutationType::Relative(0..=1))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 9]);

    // size makes error as it counts 0.0 twice, this is fine
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));

    let mut population = Population::new(vec![], true);
    genotype.fill_neighbouring_population(&chromosome, &mut population, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![vec![5, 4, 9], vec![4, 5, 9]]
    );
}

#[test]
fn integer_permutable_gene_values_scaled() {
    let scaled_ranges = &vec![-100..=100, -10..=10, -1..=1];
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=1000)
        .with_mutation_type(MutationType::Scaled(scaled_ranges.clone()))
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(inspect::chromosome(&chromosome), vec![447, 439]);

    assert_eq!(genotype.current_scale_index, 0);
    assert_eq!(
        genotype.permutable_gene_values_scaled(Some(&chromosome), scaled_ranges),
        vec![
            vec![0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000],
            vec![0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000],
        ]
    );
    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 1);
    assert_eq!(
        genotype.permutable_gene_values_scaled(Some(&chromosome), scaled_ranges),
        vec![
            vec![
                347, 357, 367, 377, 387, 397, 407, 417, 427, 437, 447, 457, 467, 477, 487, 497,
                507, 517, 527, 537, 547
            ],
            vec![
                339, 349, 359, 369, 379, 389, 399, 409, 419, 429, 439, 449, 459, 469, 479, 489,
                499, 509, 519, 529, 539
            ]
        ]
    );
    assert!(genotype.increment_scale_index());
    assert_eq!(genotype.current_scale_index, 2);
    assert_eq!(
        genotype.permutable_gene_values_scaled(Some(&chromosome), scaled_ranges),
        vec![
            vec![
                437, 438, 439, 440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452,
                453, 454, 455, 456, 457
            ],
            vec![
                429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439, 440, 441, 442, 443, 444,
                445, 446, 447, 448, 449
            ]
        ]
    );
}

#[test]
fn integer_chromosome_permutations_2_scaled() {
    let scaled_ranges = &vec![-5..=5, -2..=2, -1..=1];
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = RangeGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=10)
        .with_mutation_type(MutationType::Scaled(scaled_ranges.clone()))
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
        genotype.chromosome_permutations_size_scaled(0, scaled_ranges),
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
        genotype.chromosome_permutations_size_scaled(1, scaled_ranges),
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
        genotype.chromosome_permutations_size_scaled(2, scaled_ranges),
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
