#[cfg(test)]
use crate::support::*;
use genetic_algorithm::chromosome::ChromosomeManager;
use genetic_algorithm::genotype::{
    BitGenotype, EvolveGenotype, Genotype, HillClimbGenotype, PermutateGenotype,
};

#[test]
fn builders() {
    // all tests will fail if block size is not 64
    assert_eq!(fixedbitset::Block::BITS, 64);

    let chromosome = build::chromosome_from_str("1111100111");
    assert_eq!(inspect::chromosome_to_str(&chromosome), "1111100111");

    let chromosome = build::chromosome_from_blocks(300, [89, 51, 33, 127, 23, 18]);
    assert_eq!(
        inspect::chromosome_to_blocks(&chromosome),
        [89, 51, 33, 127, 23]
    );
}
#[test]
fn mutate_chromosome_single() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(10).build().unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome_to_str(&chromosome), "0011000100");

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome_to_str(&chromosome), "0011100100");

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome_to_str(&chromosome), "0011100101");
}
#[test]
fn mutate_chromosome_genes_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(10).build().unwrap();

    let mut chromosome = build::chromosome_from_str("1111111111");
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome_to_str(&chromosome), "1111011100");
}
#[test]
fn mutate_chromosome_genes_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(10).build().unwrap();

    let mut chromosome = build::chromosome_from_str("1111111111");
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert_eq!(inspect::chromosome_to_str(&chromosome), "1100011001");
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(10).build().unwrap();

    let mut father = build::chromosome_from_str("1111111111");
    let mut mother = build::chromosome_from_str("0000000000");
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome_to_str(&father), "1111011111");
    assert_eq!(inspect::chromosome_to_str(&mother), "0000100000");
}

#[test]
fn crossover_points() {
    let genotype = BitGenotype::builder().with_genes_size(10).build().unwrap();
    assert_eq!(genotype.crossover_points, vec![]);

    let genotype = BitGenotype::builder().with_genes_size(200).build().unwrap();
    assert_eq!(genotype.crossover_points, vec![64, 128, 192]);
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(300).build().unwrap();
    let mut father = build::chromosome_from_blocks(300, [1, 1, 1, 1, 1]);
    let mut mother = build::chromosome_from_blocks(300, [0, 0, 0, 0, 0]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome_to_blocks(&father), [1, 0, 0, 0, 0]);
    assert_eq!(inspect::chromosome_to_blocks(&mother), [0, 1, 1, 1, 1]);
}

#[test]
fn crossover_chromosome_genes_with_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(10).build().unwrap();

    let mut father = build::chromosome_from_str("1111111111");
    let mut mother = build::chromosome_from_str("0000000000");
    genotype.crossover_chromosome_genes(3, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome_to_str(&father), "1111111110");
    assert_eq!(inspect::chromosome_to_str(&mother), "0000000001");
}

#[test]
fn crossover_chromosome_genes_without_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(10).build().unwrap();

    let mut father = build::chromosome_from_str("1111111111");
    let mut mother = build::chromosome_from_str("0000000000");
    genotype.crossover_chromosome_genes(3, false, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome_to_str(&father), "1111011001");
    assert_eq!(inspect::chromosome_to_str(&mother), "0000100110");
}

#[test]
fn crossover_chromosome_points_with_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(300).build().unwrap();
    let mut father = build::chromosome_from_blocks(300, [1, 1, 1, 1, 1]);
    let mut mother = build::chromosome_from_blocks(300, [0, 0, 0, 0, 0]);
    genotype.crossover_chromosome_points(3, true, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome_to_blocks(&father), [1, 1, 1, 0, 0]);
    assert_eq!(inspect::chromosome_to_blocks(&mother), [0, 0, 0, 1, 1]);
}

#[test]
fn crossover_chromosome_points_without_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(300).build().unwrap();
    let mut father = build::chromosome_from_blocks(300, [1, 1, 1, 1, 1]);
    let mut mother = build::chromosome_from_blocks(300, [0, 0, 0, 0, 0]);
    genotype.crossover_chromosome_points(3, false, &mut father, &mut mother, rng);
    assert_eq!(inspect::chromosome_to_blocks(&father), [1, 0, 1, 0, 0]);
    assert_eq!(inspect::chromosome_to_blocks(&mother), [0, 1, 0, 1, 1]);
}

#[test]
fn neighbouring_population() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder().with_genes_size(10).build().unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(inspect::chromosome_to_str(&chromosome), "0011000100");

    assert_eq!(
        genotype.neighbouring_population_size(),
        BigUint::from(10u32)
    );

    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population_to_str(&population),
        vec![
            "1011000100",
            "0111000100",
            "0001000100",
            "0010000100",
            "0011100100",
            "0011010100",
            "0011001100",
            "0011000000",
            "0011000110",
            "0011000101",
        ]
    );
}

#[test]
fn chromosome_permutations_genes_size_1() {
    let genotype = BitGenotype::builder().with_genes_size(1).build().unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(2u32));
    assert_eq!(
        inspect::chromosomes_to_str(
            genotype
                .chromosome_permutations_into_iter()
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec!["1", "0"]
    )
}

#[test]
fn chromosome_permutations_genes_size_2() {
    let genotype = BitGenotype::builder().with_genes_size(2).build().unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(4u32));
    assert_eq!(
        inspect::chromosomes_to_str(
            genotype
                .chromosome_permutations_into_iter()
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec!["11", "10", "01", "00"]
    )
}

#[test]
fn chromosome_permutations_genes_size_huge() {
    let genotype = BitGenotype::builder().with_genes_size(100).build().unwrap();
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::parse_bytes(b"1267650600228229401496703205376", 10).unwrap()
    );

    // ensure lazy
    assert_eq!(
        inspect::chromosomes_to_blocks(
            genotype
                .chromosome_permutations_into_iter()
                .take(1)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![[18446744073709551615, 68719476735]]
    )
}

#[test]
fn chromosome_permutations_genes_size_3() {
    let genotype = BitGenotype::builder().with_genes_size(3).build().unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(8u32));
    assert_eq!(
        inspect::chromosomes_to_str(
            genotype
                .chromosome_permutations_into_iter()
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec!["111", "110", "101", "100", "011", "010", "001", "000"]
    )
}

#[test]
fn chromosome_permutations_with_seed_genes_list() {
    let genotype = BitGenotype::builder()
        .with_genes_size(6)
        .with_seed_genes_list(vec![
            BitGenotype::genes_from_str("111111"),
            BitGenotype::genes_from_str("011111"),
            BitGenotype::genes_from_str("101111"),
            BitGenotype::genes_from_str("001111"),
        ])
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(4u32));
    assert_eq!(
        inspect::chromosomes_to_str(
            genotype
                .chromosome_permutations_into_iter()
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec!["111111", "011111", "101111", "001111"]
    )
}

#[test]
fn chromosome_constructor_with_seed_genes_list() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = BitGenotype::builder()
        .with_genes_size(4)
        .with_seed_genes_list(vec![
            BitGenotype::genes_from_str("1100"),
            BitGenotype::genes_from_str("0011"),
        ])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosomes = vec![
        genotype.chromosome_constructor_random(&mut rng),
        genotype.chromosome_constructor_random(&mut rng),
        genotype.chromosome_constructor_random(&mut rng),
        genotype.chromosome_constructor_random(&mut rng),
    ];
    println!("{:#?}", chromosomes);
    assert_eq!(
        inspect::chromosomes_to_str(&chromosomes),
        vec!["0011", "1100", "0011", "1100"]
    )
}
