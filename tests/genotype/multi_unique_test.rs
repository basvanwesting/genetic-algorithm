#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    EvolveGenotype, Genotype, HillClimbGenotype, MultiUniqueGenotype, PermutateGenotype,
};

#[test]
fn sample_gene_indices() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0, 1], vec![4, 5, 6, 7], vec![0, 1, 2, 3]])
        .build()
        .unwrap();

    // disble test because HashMap does not preserve order, non repeatable test
    // assert_eq!(
    //     genotype.sample_gene_indices(10, false, &mut rng),
    //     vec![4, 2, 5, 3, 7, 6, 8, 9]
    // );
    assert_eq!(
        genotype.sample_gene_indices(10, true, &mut rng),
        vec![3, 5, 5, 5, 3, 3, 6, 9, 6, 9]
    );
}
#[test]
fn mutate_chromosome_single() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0, 1], vec![4, 5, 6, 7], vec![0, 1, 2]])
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 1, 6, 5, 4, 7, 1, 2, 0]
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 1, 6, 7, 4, 5, 1, 2, 0]
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 1, 6, 7, 4, 5, 2, 1, 0]
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 1, 6, 5, 4, 7, 2, 1, 0]
    );

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(288u32)
    );
}
#[test]
fn mutate_chromosome_genes_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![9, 8, 7],
            vec![6, 5, 4],
            vec![3, 2, 1],
        ])
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    genotype.mutate_chromosome_genes(3, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![1, 2, 3, 4, 5, 6, 7, 9, 8, 8, 7, 6, 5, 4, 3, 2, 1]
    );
}
#[test]
fn mutate_chromosome_genes_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2], //0
            vec![3, 4, 5], //1
            vec![6, 7, 8], //2
            vec![9, 8, 7], //3
            vec![6, 5, 4], //4
            vec![3, 2, 1], //5
        ])
        .build()
        .unwrap();

    assert_eq!(genotype.genes_size, 18);
    assert_eq!(genotype.allele_list_sizes, vec![3, 3, 3, 3, 3, 3]);
    assert_eq!(
        genotype.allele_list_index_offsets,
        vec![0, 3, 6, 9, 12, 15, 18]
    );
    assert_eq!(genotype.crossover_points, vec![3, 6, 9, 12, 15]);
    let mut chromosome =
        build::chromosome(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    genotype.mutate_chromosome_genes(3, false, &mut chromosome, None, &mut rng);

    assert_eq!(inspect::chromosome(&chromosome).len(), 18);

    // this test step is flaky, the result is not deterministic. Probably due to WeightedIndex f64 conversion
    // assert_eq!(
    //     inspect::chromosome(&chromosome),
    //     vec![0, 1, 2, 3, 4, 5, 6, 8, 7, 9, 8, 7, 6, 5, 4, 3, 1, 2]
    // );
}

#[test]
#[should_panic]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0, 1], vec![4, 5, 6, 7], vec![0, 1, 2]])
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![0, 1, 4, 5, 6, 7, 0, 1, 2]);
    let mut mother = build::chromosome(vec![1, 0, 5, 6, 7, 4, 1, 2, 0]);
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0, 1], vec![4, 5, 6, 7], vec![0, 1, 2]])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_list_sizes, vec![2, 4, 3]);
    assert_eq!(genotype.allele_list_index_offsets, vec![0, 2, 6, 9]);
    assert_eq!(genotype.crossover_points, vec![2, 6]);
    let mut father = build::chromosome(vec![0, 1, 4, 5, 6, 7, 0, 1, 2]);
    let mut mother = build::chromosome(vec![1, 0, 5, 6, 7, 4, 1, 2, 0]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![0, 1, 5, 6, 7, 4, 1, 2, 0]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![1, 0, 4, 5, 6, 7, 0, 1, 2]
    );
}
#[test]
fn crossover_chromosome_points_with_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1],
            vec![4, 5, 6, 7],
            vec![0, 1, 2],
            vec![9, 8, 7],
            vec![4, 5, 6],
            vec![4, 3],
        ])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_list_sizes, vec![2, 4, 3, 3, 3, 2]);
    assert_eq!(
        genotype.allele_list_index_offsets,
        vec![0, 2, 6, 9, 12, 15, 17]
    );
    assert_eq!(genotype.crossover_points, vec![2, 6, 9, 12, 15]);
    let mut father = build::chromosome(vec![0, 1, 4, 5, 6, 7, 0, 1, 2, 7, 8, 9, 4, 5, 6, 3, 4]);
    let mut mother = build::chromosome(vec![1, 0, 5, 6, 7, 4, 1, 2, 0, 9, 8, 7, 6, 5, 4, 4, 3]);
    genotype.crossover_chromosome_points(3, true, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![0, 1, 4, 5, 6, 7, 0, 1, 2, 7, 8, 9, 4, 5, 6, 4, 3]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![1, 0, 5, 6, 7, 4, 1, 2, 0, 9, 8, 7, 6, 5, 4, 3, 4]
    );
}
#[test]
fn crossover_chromosome_points_without_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1],
            vec![4, 5, 6, 7],
            vec![0, 1, 2],
            vec![9, 8, 7],
            vec![4, 5, 6],
            vec![4, 3],
        ])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_list_sizes, vec![2, 4, 3, 3, 3, 2]);
    assert_eq!(
        genotype.allele_list_index_offsets,
        vec![0, 2, 6, 9, 12, 15, 17]
    );
    assert_eq!(genotype.crossover_points, vec![2, 6, 9, 12, 15]);
    let mut father = build::chromosome(vec![0, 1, 4, 5, 6, 7, 0, 1, 2, 7, 8, 9, 4, 5, 6, 3, 4]);
    let mut mother = build::chromosome(vec![1, 0, 5, 6, 7, 4, 1, 2, 0, 9, 8, 7, 6, 5, 4, 4, 3]);
    genotype.crossover_chromosome_points(2, false, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![0, 1, 4, 5, 6, 7, 0, 1, 2, 9, 8, 7, 6, 5, 4, 3, 4]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![1, 0, 5, 6, 7, 4, 1, 2, 0, 7, 8, 9, 4, 5, 6, 4, 3]
    );
}

#[test]
fn chromosome_permutations_genes_size_1() {
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0]])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_list_sizes, vec![1]);
    assert_eq!(genotype.allele_list_index_offsets, vec![0, 1]);
    assert_eq!(genotype.crossover_points, vec![] as Vec<usize>);
    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(1u32));
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![vec![0]]
    );
}

#[test]
fn chromosome_permutations_genes_size_4() {
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1]])
        .build()
        .unwrap();

    assert_eq!(genotype.allele_list_sizes, vec![1, 2, 3, 2]);
    assert_eq!(genotype.allele_list_index_offsets, vec![0, 1, 3, 6, 8]);
    assert_eq!(genotype.crossover_points, vec![1, 3, 6]);
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(24u32)
    );
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![
            vec![0, 0, 1, 0, 1, 2, 0, 1],
            vec![0, 0, 1, 0, 1, 2, 1, 0],
            vec![0, 0, 1, 0, 2, 1, 0, 1],
            vec![0, 0, 1, 0, 2, 1, 1, 0],
            vec![0, 0, 1, 1, 0, 2, 0, 1],
            vec![0, 0, 1, 1, 0, 2, 1, 0],
            vec![0, 0, 1, 1, 2, 0, 0, 1],
            vec![0, 0, 1, 1, 2, 0, 1, 0],
            vec![0, 0, 1, 2, 0, 1, 0, 1],
            vec![0, 0, 1, 2, 0, 1, 1, 0],
            vec![0, 0, 1, 2, 1, 0, 0, 1],
            vec![0, 0, 1, 2, 1, 0, 1, 0],
            vec![0, 1, 0, 0, 1, 2, 0, 1],
            vec![0, 1, 0, 0, 1, 2, 1, 0],
            vec![0, 1, 0, 0, 2, 1, 0, 1],
            vec![0, 1, 0, 0, 2, 1, 1, 0],
            vec![0, 1, 0, 1, 0, 2, 0, 1],
            vec![0, 1, 0, 1, 0, 2, 1, 0],
            vec![0, 1, 0, 1, 2, 0, 0, 1],
            vec![0, 1, 0, 1, 2, 0, 1, 0],
            vec![0, 1, 0, 2, 0, 1, 0, 1],
            vec![0, 1, 0, 2, 0, 1, 1, 0],
            vec![0, 1, 0, 2, 1, 0, 0, 1],
            vec![0, 1, 0, 2, 1, 0, 1, 0],
        ]
    );
}

#[test]
fn chromosome_permutations_genes_size_huge() {
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![
            (0..10).collect(),
            (0..10).collect(),
            (0..10).collect(),
            (0..10).collect(),
            (0..10).collect(),
            (0..10).collect(),
        ])
        .build()
        .unwrap();
    assert_eq!(genotype.allele_list_sizes, vec![10, 10, 10, 10, 10, 10]);
    assert_eq!(
        genotype.allele_list_index_offsets,
        vec![0, 10, 20, 30, 40, 50, 60]
    );
    assert_eq!(genotype.crossover_points, vec![10, 20, 30, 40, 50]);
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::parse_bytes(b"2283380023591730815784976384000000000000", 10).unwrap()
    );

    // ensure lazy
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .take(1)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8,
            9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7,
            8, 9,
        ]]
    )
}

#[test]
fn neighbouring_population_4() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = MultiUniqueGenotype::builder()
        .with_allele_lists(vec![vec![0], vec![0, 1], vec![0, 1, 2], vec![0, 1]])
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![0, 0, 1, 2, 0, 1, 0, 1]
    );

    assert_eq!(genotype.neighbouring_population_size(), BigUint::from(5u32));
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![0, 1, 0, 2, 0, 1, 0, 1],
            vec![0, 0, 1, 0, 2, 1, 0, 1],
            vec![0, 0, 1, 1, 0, 2, 0, 1],
            vec![0, 0, 1, 2, 1, 0, 0, 1],
            vec![0, 0, 1, 2, 0, 1, 1, 0]
        ]
    );
}
