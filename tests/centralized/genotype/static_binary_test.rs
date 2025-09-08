#[cfg(test)]
use crate::support::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::{
    EvolveGenotype, Genotype, HillClimbGenotype, StaticBinaryGenotype,
};

#[test]
fn mutate_chromosome_single() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosome),
        vec![false, false, true, false, true, true, true, false, false, true]
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosome),
        vec![false, false, false, false, true, true, true, false, false, true]
    );

    genotype.mutate_chromosome_genes(1, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosome),
        vec![false, false, false, false, true, true, true, false, false, false]
    );
}
#[test]
fn mutate_chromosome_genes_with_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = static_build::chromosome(&mut genotype, vec![true; 10]);
    genotype.mutate_chromosome_genes(5, true, &mut chromosome, None, &mut rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosome),
        vec![true, true, true, true, false, true, true, true, false, false]
    );
}
#[test]
fn mutate_chromosome_genes_without_duplicates() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosome = static_build::chromosome(&mut genotype, vec![true; 10]);
    genotype.mutate_chromosome_genes(5, false, &mut chromosome, None, &mut rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosome),
        vec![true, true, false, false, false, true, true, false, false, true]
    );
}

#[test]
fn crossover_chromosome_pair_single_gene() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut father = static_build::chromosome(&mut genotype, vec![true; 10]);
    let mut mother = static_build::chromosome(&mut genotype, vec![false; 10]);
    genotype.crossover_chromosome_genes(1, true, &mut father, &mut mother, rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &father),
        vec![true, true, true, true, false, true, true, true, true, true]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &mother),
        vec![false, false, false, false, true, false, false, false, false, false]
    );
}

#[test]
fn crossover_chromosome_pair_single_point() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut father = static_build::chromosome(&mut genotype, vec![true; 10]);
    let mut mother = static_build::chromosome(&mut genotype, vec![false; 10]);
    genotype.crossover_chromosome_points(1, true, &mut father, &mut mother, rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &father),
        vec![true, true, true, true, false, false, false, false, false, false]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &mother),
        vec![false, false, false, false, true, true, true, true, true, true]
    );
}

#[test]
fn crossover_chromosome_genes_with_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut father = static_build::chromosome(&mut genotype, vec![true; 10]);
    let mut mother = static_build::chromosome(&mut genotype, vec![false; 10]);
    genotype.crossover_chromosome_genes(3, true, &mut father, &mut mother, rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &father),
        vec![true, true, true, true, true, true, true, true, true, false]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &mother),
        vec![false, false, false, false, false, false, false, false, false, true]
    );
}

#[test]
fn crossover_chromosome_genes_without_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut father = static_build::chromosome(&mut genotype, vec![true; 10]);
    let mut mother = static_build::chromosome(&mut genotype, vec![false; 10]);
    genotype.crossover_chromosome_genes(3, false, &mut father, &mut mother, rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &father),
        vec![true, true, true, true, false, true, true, false, false, true]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &mother),
        vec![false, false, false, false, true, false, false, true, true, false]
    );
}

#[test]
fn crossover_chromosome_points_with_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut father = static_build::chromosome(&mut genotype, vec![true; 10]);
    let mut mother = static_build::chromosome(&mut genotype, vec![false; 10]);
    genotype.crossover_chromosome_points(3, true, &mut father, &mut mother, rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &father),
        vec![true, true, true, true, true, true, true, true, true, false]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &mother),
        vec![false, false, false, false, false, false, false, false, false, true]
    );
}

#[test]
fn crossover_chromosome_points_without_duplicates() {
    let rng = &mut SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 5>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut father = static_build::chromosome(&mut genotype, vec![true; 10]);
    let mut mother = static_build::chromosome(&mut genotype, vec![false; 10]);
    genotype.crossover_chromosome_points(3, false, &mut father, &mut mother, rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &father),
        vec![true, true, true, true, false, false, false, true, false, false]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &mother),
        vec![false, false, false, false, true, true, true, false, true, true]
    );
}

#[test]
fn neighbouring_population() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<10, 11>::builder()
        .with_genes_size(10)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome = genotype.chromosome_constructor_random(&mut rng);
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosome),
        vec![false, false, true, false, true, true, true, false, false, true]
    );

    assert_eq!(
        genotype.neighbouring_population_size(),
        BigUint::from(10u32)
    );
    let mut population = Population::new(vec![]);
    genotype.fill_neighbouring_population(&chromosome, &mut population, None, &mut rng);
    assert_eq!(
        static_inspect::population(&genotype, &population),
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
fn chromosome_constructor_with_seed_genes_list() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<4, 5>::builder()
        .with_genes_size(4)
        .with_seed_genes_list(vec![
            Box::new([true, true, false, false]),
            Box::new([false, false, true, true]),
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
        static_inspect::chromosome(&genotype, &chromosomes[0]),
        vec![false, false, true, true]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosomes[1]),
        vec![true, true, false, false]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosomes[2]),
        vec![false, false, true, true]
    );
    assert_eq!(
        static_inspect::chromosome(&genotype, &chromosomes[3]),
        vec![true, true, false, false]
    );
}

#[test]
fn population_constructor_random() {
    let mut rng = SmallRng::seed_from_u64(0);
    let mut genotype = StaticBinaryGenotype::<4, 5>::builder()
        .with_genes_size(4)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = genotype.population_constructor(5, &mut rng);
    println!("{:#?}", population.chromosomes);
    assert_eq!(
        static_inspect::population(&genotype, &population),
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
    let mut genotype = StaticBinaryGenotype::<4, 5>::builder()
        .with_genes_size(4)
        .with_seed_genes_list(vec![
            Box::new([true, true, false, false]),
            Box::new([false, false, true, true]),
        ])
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let population = genotype.population_constructor(5, &mut rng);
    println!("{:#?}", population.chromosomes);
    assert_eq!(
        static_inspect::population(&genotype, &population),
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
    let mut genotype = StaticBinaryGenotype::<5, 5>::builder()
        .with_genes_size(5)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let mut chromosomes = (0..4)
        .map(|_| genotype.chromosome_constructor_random(rng))
        .collect::<Vec<_>>();
    genotype.save_best_genes(&chromosomes[2]);
    dbg!("init", &chromosomes, &genotype.best_genes());

    assert_eq!(
        static_inspect::chromosomes(&genotype, &chromosomes),
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
            vec![false, true, true, false, true],
            vec![false, false, false, true, true],
        ]
    );
    assert_eq!(
        genotype.best_genes().to_vec(),
        vec![false, true, true, false, true],
    );

    genotype.chromosome_destructor_truncate(&mut chromosomes, 2);
    dbg!("truncate", &chromosomes, &genotype.best_genes());

    assert_eq!(
        static_inspect::chromosomes(&genotype, &chromosomes),
        vec![
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
        ]
    );

    genotype.chromosome_cloner_expand(&mut chromosomes, 2);
    dbg!("clone range", &chromosomes, &genotype.best_genes());

    assert_eq!(
        static_inspect::chromosomes(&genotype, &chromosomes),
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
    dbg!("mutate", &chromosomes, &genotype.best_genes());

    assert_eq!(
        static_inspect::chromosomes(&genotype, &chromosomes),
        vec![
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, false, true, false, true],
            vec![true, true, false, false, true],
        ]
    );
    assert_eq!(
        genotype.best_genes().to_vec(),
        vec![false, true, true, false, true],
    );
}

#[test]
fn calculate_genes_hash() {
    let mut genotype = StaticBinaryGenotype::<3, 5>::builder()
        .with_genes_size(3)
        .with_genes_hashing(true)
        .build()
        .unwrap();
    genotype.chromosomes_setup();

    let chromosome_1 = static_build::chromosome(&mut genotype, vec![true, true, true]);
    let chromosome_2 = static_build::chromosome(&mut genotype, vec![true, true, true]);
    let chromosome_3 = static_build::chromosome(&mut genotype, vec![true, false, true]);
    let chromosome_4 = static_build::chromosome(&mut genotype, vec![true, false, true]);

    assert!(genotype.calculate_genes_hash(&chromosome_1).is_some());
    // assert_eq!(
    //     genotype.calculate_genes_hash(&chromosome_1),
    //     Some(1044924641990395411)
    // );
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_1),
        genotype.calculate_genes_hash(&chromosome_2),
    );
    assert_eq!(
        genotype.calculate_genes_hash(&chromosome_3),
        genotype.calculate_genes_hash(&chromosome_4),
    );

    assert_ne!(
        genotype.calculate_genes_hash(&chromosome_1),
        genotype.calculate_genes_hash(&chromosome_3),
    );
}
