#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    BinaryGenotype, Genotype, IncrementalGenotype, PermutableGenotype,
};

#[test]
fn general() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, false, true, false, false, false, true, true, false]
    );

    genotype.mutate_chromosome_random(&mut chromosome, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, true, true, false, false, false, true, true, false]
    );

    genotype.mutate_chromosome_neighbour(&mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, true, true, false, false, false, true, true, true]
    );

    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::from(1024u32)
    );
    assert_eq!(genotype.crossover_indexes(), (0..10).collect::<Vec<_>>());
    assert_eq!(genotype.crossover_points(), (0..10).collect::<Vec<_>>());
}

#[test]
fn neighbouring_population() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let chromosome = genotype.chromosome_factory(&mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, false, true, false, false, false, true, true, false]
    );

    assert_eq!(
        genotype.neighbouring_population_size(),
        BigUint::from(10u32)
    );
    assert_eq!(
        inspect::population(&genotype.neighbouring_population(&chromosome, None)),
        vec![
            vec![false, true, false, true, false, false, false, true, true, false],
            vec![true, false, false, true, false, false, false, true, true, false],
            vec![true, true, true, true, false, false, false, true, true, false],
            vec![true, true, false, false, false, false, false, true, true, false],
            vec![true, true, false, true, true, false, false, true, true, false],
            vec![true, true, false, true, false, true, false, true, true, false],
            vec![true, true, false, true, false, false, true, true, true, false],
            vec![true, true, false, true, false, false, false, false, true, false],
            vec![true, true, false, true, false, false, false, true, false, false],
            vec![true, true, false, true, false, false, false, true, true, true],
        ]
    );
}

#[test]
fn chromosome_permutations_genes_size_1() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(1)
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(2u32));
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![vec![true], vec![false],]
    )
}

#[test]
fn chromosome_permutations_genes_size_2() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(2)
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(4u32));
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![
            vec![true, true],
            vec![true, false],
            vec![false, true],
            vec![false, false],
        ]
    )
}

#[test]
fn chromosome_permutations_genes_size_huge() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(100)
        .build()
        .unwrap();
    assert_eq!(
        genotype.chromosome_permutations_size(),
        BigUint::parse_bytes(b"1267650600228229401496703205376", 10).unwrap()
    );
}

#[test]
fn chromosome_permutations_genes_size_3() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(8u32));
    assert_eq!(
        inspect::chromosomes(&genotype.chromosome_permutations_into_iter().collect()),
        vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, true],
            vec![true, false, false],
            vec![false, true, true],
            vec![false, true, false],
            vec![false, false, true],
            vec![false, false, false],
        ]
    )
}

#[test]
fn chromosome_factory_with_seed_genes_list() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(4)
        .with_seed_genes_list(vec![
            vec![true, true, false, false],
            vec![false, false, true, true],
        ])
        .build()
        .unwrap();
    let chromosomes = vec![
        genotype.chromosome_factory(&mut rng),
        genotype.chromosome_factory(&mut rng),
        genotype.chromosome_factory(&mut rng),
        genotype.chromosome_factory(&mut rng),
    ];
    println!("{:#?}", chromosomes);
    assert_eq!(
        inspect::chromosome(&chromosomes[0]),
        vec![false, false, true, true]
    );
    assert_eq!(
        inspect::chromosome(&chromosomes[1]),
        vec![true, true, false, false]
    );
    assert_eq!(
        inspect::chromosome(&chromosomes[2]),
        vec![false, false, true, true]
    );
    assert_eq!(
        inspect::chromosome(&chromosomes[3]),
        vec![true, true, false, false]
    );
}
