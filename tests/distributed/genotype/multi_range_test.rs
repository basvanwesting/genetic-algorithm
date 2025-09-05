#[cfg(test)]
use crate::support::*;
use genetic_algorithm::distributed::chromosome::ChromosomeManager;
use genetic_algorithm::distributed::genotype::{
    EvolveGenotype, Genotype, HillClimbGenotype, MultiRangeGenotype, PermutateGenotype,
};

#[test]
fn float_mutate_chromosome_single_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.195, 19.798],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 4.485, 19.798],
        0.001
    ));
}

#[test]
fn float_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_mutation_ranges(vec![-0.1..=0.1, -0.5..=0.5, -1.0..=1.0])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 19.798],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.592, 19.798],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.487, 19.975],
        0.001
    ));
}

#[test]
fn float_mutate_chromosome_single_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_mutation_scaled_ranges(vec![
            vec![-0.5..=0.5, -1.0..=1.0, -5.0..=5.0],
            vec![-0.1..=0.1, -0.5..=0.5, -1.0..=1.0],
            vec![-0.01..=0.01, -0.05..=0.05, -0.1..=0.1],
        ])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.195, 19.798],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.145, 19.798],
        0.001
    ));

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.145, 19.698],
        0.001
    ));
}

#[test]
fn mutate_chromosome_genes_random_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0.0, 0.0, 10.0]);
    genotype.mutate_chromosome_genes(3, true, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.0, 2.195, 19.429],
        0.001
    ));
}
#[test]
fn mutate_chromosome_genes_random_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![0.0, 0.0, 10.0]);
    genotype.mutate_chromosome_genes(2, false, &mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.0, 4.094, 13.951],
        0.001
    ));
}

#[test]
fn float_crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0.1, 1.1, 10.1]);
    let mut mother = build::chromosome(vec![0.9, 3.9, 18.9]);
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome(&father), vec![0.1, 3.9, 10.1]);
    assert_eq!(inspect::chromosome(&mother), vec![0.9, 1.1, 18.9]);
}

#[test]
fn float_crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0.1, 1.1, 10.1]);
    let mut mother = build::chromosome(vec![0.9, 3.9, 18.9]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome(&father), vec![0.1, 3.9, 18.9]);
    assert_eq!(inspect::chromosome(&mother), vec![0.9, 1.1, 10.1]);
}

#[test]
fn float_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0])
        .with_allele_mutation_ranges(vec![-0.1..=0.1])
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
        0.001
    ));
}

#[test]
fn float_neighbouring_population_3_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 19.798],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.206, 2.195, 19.798],
            vec![0.943, 2.195, 19.798],
            vec![0.447, 2.070, 19.798],
            vec![0.447, 3.845, 19.798],
            vec![0.447, 2.195, 14.471],
            vec![0.447, 2.195, 19.878],
        ],
        0.001
    ));
}

#[test]
fn float_neighbouring_population_3_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_mutation_ranges(vec![-0.1..=0.1, -0.5..=0.5, -1.0..=1.0])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 19.798],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.394, 2.196, 19.799],
            vec![0.537, 2.196, 19.799],
            vec![0.447, 2.167, 19.799],
            vec![0.447, 2.490, 19.799],
            vec![0.447, 2.196, 19.255],
            vec![0.447, 2.196, 19.878],
        ],
        0.001
    ));
}

#[test]
fn float_neighbouring_population_3_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=1.0, 0.0..=5.0, 10.0..=20.0])
        .with_allele_mutation_scaled_ranges(vec![
            vec![-0.5..=0.5, -1.0..=1.0, -5.0..=5.0],
            vec![-0.1..=0.1, -0.5..=0.5, -1.0..=1.0],
            vec![-0.01..=0.01, -0.05..=0.05, -0.1..=0.1],
        ])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 2.196, 19.798],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(0), &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.0, 2.196, 19.799],
            vec![0.947, 2.196, 19.799],
            vec![0.447, 1.196, 19.799],
            vec![0.447, 3.196, 19.799],
            vec![0.447, 2.196, 14.799],
            vec![0.447, 2.196, 20.0],
        ],
        0.001
    ));
    genotype.chromosome_destructor_truncate(&mut population.chromosomes, 0);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(1), &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.347, 2.196, 19.799],
            vec![0.547, 2.196, 19.799],
            vec![0.447, 1.696, 19.799],
            vec![0.447, 2.696, 19.799],
            vec![0.447, 2.196, 18.799],
            vec![0.447, 2.196, 20.0]
        ],
        0.001
    ));
    genotype.chromosome_destructor_truncate(&mut population.chromosomes, 0);
    genotype.fill_neighbouring_population(&chromosome, &mut population, Some(2), &mut rng);
    assert!(relative_population_eq(
        inspect::population(&population),
        vec![
            vec![0.437, 2.196, 19.799],
            vec![0.457, 2.196, 19.799],
            vec![0.447, 2.146, 19.799],
            vec![0.447, 2.246, 19.799],
            vec![0.447, 2.196, 19.699],
            vec![0.447, 2.196, 19.899],
        ],
        0.001
    ));
}

#[test]
fn float_permutable_gene_values_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=10.0, 0.0..=5.0])
        .with_allele_mutation_scaled_ranges(vec![
            vec![-1.0..=1.0, -1.0..=1.0],
            vec![-0.1..=0.1, -0.2..=0.2],
            vec![-0.01..=0.01, -0.05..=0.05],
        ])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![4.473, 2.195],
        0.001,
    ));

    assert!(relative_population_eq(
        genotype.permutable_gene_values_scaled(Some(&chromosome), 0),
        vec![
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
        ],
        0.001
    ));

    assert!(relative_population_eq(
        genotype.permutable_gene_values_scaled(Some(&chromosome), 1),
        vec![
            vec![
                3.473, 3.573, 3.673, 3.773, 3.873, 3.973, 4.073, 4.173, 4.273, 4.373, 4.473, 4.573,
                4.673, 4.773, 4.873, 4.973, 5.073, 5.173, 5.273, 5.373, 5.473, 5.473,
            ],
            vec![1.195, 1.395, 1.595, 1.795, 1.995, 2.195, 2.395, 2.595, 2.795, 2.995, 3.195]
        ],
        0.001
    ));

    assert!(relative_population_eq(
        genotype.permutable_gene_values_scaled(Some(&chromosome), 2),
        vec![
            vec![
                4.373, 4.383, 4.393, 4.403, 4.413, 4.423, 4.433, 4.443, 4.453, 4.463, 4.473, 4.483,
                4.493, 4.503, 4.513, 4.523, 4.533, 4.543, 4.553, 4.563, 4.573, 4.573,
            ],
            vec![1.995, 2.045, 2.095, 2.145, 2.195, 2.245, 2.295, 2.345, 2.395, 2.395,]
        ],
        0.001
    ));
}

#[test]
fn float_chromosome_permutations_2_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0.0..=10.0, 0.0..=5.0])
        .with_allele_mutation_scaled_ranges(vec![
            vec![-5.0..=5.0, -3.0..=3.0],
            vec![-2.5..=2.5, -1.5..=1.5],
            vec![-1.0..=1.0, -1.0..=1.0],
        ])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(58u32)
    );

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![4.473, 2.195],
        0.001
    ));

    assert_eq!(
        genotype.chromosome_permutations_size_scaled(0),
        BigUint::from(9u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome), Some(0))
        .collect::<Vec<_>>();

    assert!(relative_population_eq(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0.0, 0.0],
            vec![0.0, 3.0],
            vec![0.0, 5.0],
            vec![5.0, 0.0],
            vec![5.0, 3.0],
            vec![5.0, 5.0],
            vec![10.0, 0.0],
            vec![10.0, 3.0],
            vec![10.0, 5.0],
        ],
        0.001,
    ));

    assert_eq!(
        genotype.chromosome_permutations_size_scaled(1),
        BigUint::from(25u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome), Some(1))
        .collect::<Vec<_>>();

    assert!(relative_population_eq(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![0.0, 0.0],
            vec![0.0, 1.5],
            vec![0.0, 3.0],
            vec![0.0, 4.5],
            vec![0.0, 5.0],
            vec![2.5, 0.0],
            vec![2.5, 1.5],
            vec![2.5, 3.0],
            vec![2.5, 4.5],
            vec![2.5, 5.0],
            vec![5.0, 0.0],
            vec![5.0, 1.5],
            vec![5.0, 3.0],
            vec![5.0, 4.5],
            vec![5.0, 5.0],
            vec![7.5, 0.0],
            vec![7.5, 1.5],
            vec![7.5, 3.0],
            vec![7.5, 4.5],
            vec![7.5, 5.0],
            vec![9.473, 0.0],
            vec![9.473, 1.5],
            vec![9.473, 3.0],
            vec![9.473, 4.5],
            vec![9.473, 5.0],
        ],
        0.001,
    ));

    assert_eq!(
        genotype.chromosome_permutations_size_scaled(2),
        BigUint::from(24u32)
    );
    let chromosomes = genotype
        .chromosome_permutations_into_iter(Some(&chromosome), Some(2))
        .collect::<Vec<_>>();

    assert!(relative_population_eq(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![1.973, 0.695],
            vec![1.973, 1.695],
            vec![1.973, 2.695],
            vec![1.973, 3.695],
            vec![2.973, 0.695],
            vec![2.973, 1.695],
            vec![2.973, 2.695],
            vec![2.973, 3.695],
            vec![3.973, 0.695],
            vec![3.973, 1.695],
            vec![3.973, 2.695],
            vec![3.973, 3.695],
            vec![4.973, 0.695],
            vec![4.973, 1.695],
            vec![4.973, 2.695],
            vec![4.973, 3.695],
            vec![5.973, 0.695],
            vec![5.973, 1.695],
            vec![5.973, 2.695],
            vec![5.973, 3.695],
            vec![6.973, 0.695],
            vec![6.973, 1.695],
            vec![6.973, 2.695],
            vec![6.973, 3.695],
        ],
        0.001,
    ));
}

#[test]
fn integer_mutate_chromosome_single_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 5, 20]);
}

#[test]
fn integer_mutate_chromosome_single_relative() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .with_allele_mutation_ranges(vec![-1..=1, -2..=2, -3..=3])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 20]);
}

#[test]
fn integer_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0..=9])
        .with_allele_mutation_ranges(vec![-1..=1])
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
fn integer_neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![0..=9, 0..=5, 10..=20])
        .with_allele_mutation_ranges(vec![-1..=1, -2..=2, -3..=3])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 2, 20]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![3, 2, 20],
            vec![5, 2, 20],
            vec![4, 0, 20],
            vec![4, 4, 20],
            vec![4, 2, 17],
        ]
    );
}
