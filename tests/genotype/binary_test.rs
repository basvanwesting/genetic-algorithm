#[cfg(test)]
use crate::support::*;
use genetic_algorithm::genotype::{
    BinaryGenotype, EvolveGenotype, Genotype, HillClimbGenotype, PermutateGenotype,
};

#[test]
fn mutate_chromosome_single() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![false, false, true, false, true, true, true, false, false, true]
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![false, false, false, false, true, true, true, false, false, true]
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![false, false, false, false, true, true, true, false, false, false]
    );
}
#[test]
fn mutate_chromosome_genes_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![true; 10]);
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, true, true, false, true, true, true, false, false]
    );
}
#[test]
fn mutate_chromosome_genes_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut chromosome = build::chromosome(vec![true; 10]);
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![true, true, false, false, false, true, true, false, false, true]
    );
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![true; 10]);
    let mut mother = build::chromosome(vec![false; 10]);
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![true, true, true, true, false, true, true, true, true, true]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![false, false, false, false, true, false, false, false, false, false]
    );
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![true; 10]);
    let mut mother = build::chromosome(vec![false; 10]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![true, true, true, true, false, false, false, false, false, false]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![false, false, false, false, true, true, true, true, true, true]
    );
}

#[test]
fn crossover_chromosome_genes_with_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![true; 10]);
    let mut mother = build::chromosome(vec![false; 10]);
    genotype.crossover_chromosome_genes(3, true, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![true, true, true, true, true, true, true, true, true, false]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![false, false, false, false, false, false, false, false, false, true]
    );
}

#[test]
fn crossover_chromosome_genes_without_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![true; 10]);
    let mut mother = build::chromosome(vec![false; 10]);
    genotype.crossover_chromosome_genes(3, false, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![true, true, true, true, false, true, true, false, false, true]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![false, false, false, false, true, false, false, true, true, false]
    );
}

#[test]
fn crossover_chromosome_points_with_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![true; 10]);
    let mut mother = build::chromosome(vec![false; 10]);
    genotype.crossover_chromosome_points(3, true, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![true, true, true, true, true, true, true, true, true, false]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![false, false, false, false, false, false, false, false, false, true]
    );
}

#[test]
fn crossover_chromosome_points_without_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let mut father = build::chromosome(vec![true; 10]);
    let mut mother = build::chromosome(vec![false; 10]);
    genotype.crossover_chromosome_points(3, false, &mut father, &mut mother, rng);
    assert_eq!(
        inspect::chromosome(&father),
        vec![true, true, true, true, false, false, false, true, false, false]
    );
    assert_eq!(
        inspect::chromosome(&mother),
        vec![false, false, false, false, true, true, true, false, true, true]
    );
}

#[test]
fn neighbouring_population() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(10)
        .build()
        .unwrap();

    let chromosome = Chromosome::new(genotype.random_genes_factory(&mut rng));
    assert_eq!(
        inspect::chromosome(&chromosome),
        vec![false, false, true, false, true, true, true, false, false, true]
    );

    assert_eq!(
        genotype.neighbouring_population_size(),
        BigUint::from(10u32)
    );
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, false, true, false, true, true, true, false, false, true],
            vec![false, true, true, false, true, true, true, false, false, true],
            vec![false, false, false, false, true, true, true, false, false, true],
            vec![false, false, true, true, true, true, true, false, false, true],
            vec![false, false, true, false, false, true, true, false, false, true],
            vec![false, false, true, false, true, false, true, false, false, true],
            vec![false, false, true, false, true, true, false, false, false, true],
            vec![false, false, true, false, true, true, true, true, false, true],
            vec![false, false, true, false, true, true, true, false, true, true],
            vec![false, false, true, false, true, true, true, false, false, false],
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
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .collect::<Vec<_>>()
                .as_slice()
        ),
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
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .collect::<Vec<_>>()
                .as_slice()
        ),
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

    // ensure lazy
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .take(1)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![vec![true; 100]]
    )
}

#[test]
fn chromosome_permutations_genes_size_3() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(3)
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(8u32));
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .collect::<Vec<_>>()
                .as_slice()
        ),
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
fn chromosome_permutations_with_seed_genes_list() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(6)
        .with_seed_genes_list(vec![
            vec![true, true, true, true, true, true],
            vec![false, true, true, true, true, true],
            vec![true, false, true, true, true, true],
            vec![false, false, true, true, true, true],
        ])
        .build()
        .unwrap();

    assert_eq!(genotype.chromosome_permutations_size(), BigUint::from(4u32));
    assert_eq!(
        inspect::chromosomes(
            genotype
                .chromosome_permutations_into_iter(None, None)
                .collect::<Vec<_>>()
                .as_slice()
        ),
        vec![
            vec![true, true, true, true, true, true],
            vec![false, true, true, true, true, true],
            vec![true, false, true, true, true, true],
            vec![false, false, true, true, true, true],
        ]
    )
}

#[test]
fn chromosome_constructor_with_seed_genes_list() {
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
        Chromosome::new(genotype.random_genes_factory(&mut rng)),
        Chromosome::new(genotype.random_genes_factory(&mut rng)),
        Chromosome::new(genotype.random_genes_factory(&mut rng)),
        Chromosome::new(genotype.random_genes_factory(&mut rng)),
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

#[test]
fn population_constructor_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(4)
        .build()
        .unwrap();
    let population = genotype.population_constructor(5, &mut rng);
    println!("{:#?}", population.chromosomes);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![false, false, true, false],
            vec![true, true, true, false],
            vec![false, true, false, true],
            vec![true, false, true, false],
            vec![false, false, true, true],
        ]
    )
}

#[test]
fn population_constructor_with_seed_genes_list() {
    let mut rng = SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(4)
        .with_seed_genes_list(vec![
            vec![true, true, false, false],
            vec![false, false, true, true],
        ])
        .build()
        .unwrap();
    let population = genotype.population_constructor(5, &mut rng);
    println!("{:#?}", population.chromosomes);
    assert_eq!(
        inspect::population(&population),
        vec![
            vec![true, true, false, false],
            vec![false, false, true, true],
            vec![true, true, false, false],
            vec![false, false, true, true],
            vec![true, true, false, false],
        ]
    )
}

#[test]
fn chromosome_manager() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let mut chromosomes = (0..4)
        .map(|_| Chromosome::new(genotype.random_genes_factory(rng)))
        .collect::<Vec<_>>();

    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
            vec![false, true, true, false, true],
            vec![false, false, false, true, true],
        ]
    );

    chromosomes.truncate(2);

    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
        ]
    );

    // Manually expand chromosomes (formerly chromosome_cloner_expand)
    let modulo = chromosomes.len();
    for i in 0..2 {
        let chromosome = chromosomes[i % modulo].clone();
        chromosomes.push(chromosome);
    }

    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
        ]
    );

    chromosomes
        .iter_mut()
        .take(2)
        .for_each(|c| genotype.mutate_chromosome_genes(3, false, c, None, rng));

    assert_eq!(
        inspect::chromosomes(&chromosomes),
        vec![
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
        ]
    );
}

#[test]
fn calculate_genes_hash() {
    let chromosome_1: Chromosome<bool> =
        build::chromosome_without_genes_hash(vec![true, true, true]);
    let chromosome_2: Chromosome<bool> =
        build::chromosome_without_genes_hash(vec![true, true, true]);
    let chromosome_3: Chromosome<bool> =
        build::chromosome_without_genes_hash(vec![true, false, true]);
    let chromosome_4: Chromosome<bool> =
        build::chromosome_without_genes_hash(vec![true, false, true]);

    let hash_1 = chromosome_1.calculate_hash();
    let hash_2 = chromosome_2.calculate_hash();
    let hash_3 = chromosome_3.calculate_hash();
    let hash_4 = chromosome_4.calculate_hash();

    // Same genes should have same hash
    assert_eq!(hash_1, hash_2);
    assert_eq!(hash_3, hash_4);

    // Different genes should have different hash
    assert_ne!(hash_1, hash_3);
}
