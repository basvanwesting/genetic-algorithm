#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{ContinuousGenotype, Genotype, IncrementalGenotype};

#[test]
fn float_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 0.9798802, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509,
        ],
        0.001,
    ));

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 0.9763819, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509,
        ],
        0.001,
    ));

    assert_eq!(genotype.crossover_indexes(), (0..10).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<_>>());
}

#[test]
fn float_neighbour_unscaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-1.0..=1.0)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 0.9798802, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509,
        ],
        0.001,
    ));

    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 1.0, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509,
        ],
        0.001,
    ));

    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    println!("{:?}", inspect::chromosome(&chromosome));
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 1.0, 0.46216714, 0.897079, 0.9429498, 0.0, 0.4563719, 0.3951441,
            0.8188509,
        ],
        0.001,
    ));

    assert_eq!(genotype.crossover_indexes(), (0..10).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<_>>());
}

#[test]
fn float_neighbour_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_scaled_range(vec![-1.0..=1.0, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 0.9798802, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509,
        ],
        0.001,
    ));

    genotype.mutate_chromosome_neighbour(&mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.4391402, 0.9898802, 0.46216714, 0.897079, 0.9429498, 0.5881474, 0.4563719,
            0.3951441, 0.8188509,
        ],
        0.001,
    ));

    genotype.mutate_chromosome_neighbour(&mut chromosome, Some(2), &mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![
            0.447325, 0.439140, 0.989880, 0.462167, 0.897079, 0.942949, 0.578147, 0.456371,
            0.395144, 0.818850
        ],
        0.001,
    ));

    assert_eq!(genotype.crossover_indexes(), (0..10).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<_>>());
}

#[test]
fn float_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447325],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
        vec![vec![0.391], vec![0.545]],
        0.001,
    ));
}

#[test]
fn float_neighbouring_population_2_unscaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
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
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_scaled_range(vec![-0.5..=0.5, -0.1..=0.1, -0.01..=0.01])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439],
        0.001
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, Some(0), &mut rng)),
        vec![
            vec![0.0, 0.439],
            vec![0.947, 0.439],
            vec![0.447, 0.0],
            vec![0.447, 0.939],
        ],
        0.001,
    ));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, Some(1), &mut rng)),
        vec![
            vec![0.347, 0.439],
            vec![0.547, 0.439],
            vec![0.447, 0.339],
            vec![0.447, 0.539],
        ],
        0.001,
    ));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, Some(2), &mut rng)),
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
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.980],
        0.001,
    ));

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
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
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0.0..=1.0)
        .with_allele_neighbour_range(0.0..=0.1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert!(relative_chromosome_eq(
        inspect::chromosome(&chromosome),
        vec![0.447, 0.439, 0.980],
        0.001,
    ));

    // size makes error as it counts 0.0 twice, this is fine
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    assert!(relative_population_eq(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
        vec![
            vec![0.494, 0.439, 0.980],
            vec![0.447, 0.529, 0.980],
            vec![0.447, 0.439, 0.999],
        ],
        0.001,
    ));
}

#[test]
fn integer_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 5, 4, 3, 8],
    );

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 0, 4, 3, 8],
    );

    assert_eq!(genotype.crossover_indexes(), (0..10).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<_>>());
}

#[test]
fn integer_neighbour() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(10)
        .with_allele_range(0..=9)
        .with_allele_neighbour_range(-1..=1)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 5, 4, 3, 8],
    );

    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![4, 4, 9, 4, 8, 9, 4, 4, 3, 8],
    );

    assert_eq!(genotype.crossover_indexes(), (0..10).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<_>>());
}

#[test]
fn integer_neighbouring_population_1() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(1)
        .with_allele_range(0..=9)
        .with_allele_neighbour_range(-1..=1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(2u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
        vec![vec![3], vec![5]],
    );
}

#[test]
fn integer_neighbouring_population_2_unscaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_allele_neighbour_range(-2..=2)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4],);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
        vec![vec![3, 4], vec![5, 4], vec![4, 2], vec![4, 6]]
    );
}

#[test]
fn integer_neighbouring_population_2_scaled() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(2)
        .with_allele_range(0..=9)
        .with_allele_neighbour_scaled_range(vec![-3..=3, -2..=2, -1..=1])
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(4u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, Some(1), &mut rng)),
        vec![vec![2, 4], vec![6, 4], vec![4, 2], vec![4, 6]]
    );
}

#[test]
fn integer_neighbouring_population_3() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0..=9)
        .with_allele_neighbour_range(-1..=1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 9]);

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
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
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(3)
        .with_allele_range(0..=9)
        .with_allele_neighbour_range(0..=1)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(inspect::chromosome(&chromosome), vec![4, 4, 9]);

    // size makes error as it counts 0.0 twice, this is fine
    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(6u32));

    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None, &mut rng)),
        vec![vec![5, 4, 9], vec![4, 5, 9]]
    );
}
