#[cfg(test)]
use crate::support::*;
use genetic_algorithm::fitness::placeholders::{
    CountTrue, SumDiscreteGenotype, SumMultiDiscreteGenotype,
};
use genetic_algorithm::fitness::FitnessOrdering;
use genetic_algorithm::genotype::{
    BinaryGenotype, DiscreteGenotype, Genotype, MultiDiscreteGenotype,
};
use genetic_algorithm::strategy::permutate::{Permutate, PermutateReporterNoop};
use genetic_algorithm::strategy::Strategy;

//#[test]
//build_invalid cannot be tested because invalid doesn't even have a type

#[test]
fn call_binary_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        // .with_reporter(PermutateReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = permutate.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(5));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![true, true, true, true, true]
    );
}

#[test]
fn call_binary_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_fitness(CountTrue)
        .with_reporter(PermutateReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = permutate.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(0));
    assert_eq!(
        inspect::chromosome(&best_chromosome),
        vec![false, false, false, false, false]
    );
}

#[test]
fn call_discrete() {
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(5)
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumDiscreteGenotype)
        // .with_reporter(PermutateReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = permutate.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(45));
    assert_eq!(inspect::chromosome(&best_chromosome), vec![9, 9, 9, 9, 9]);
}

#[test]
fn call_multi_discrete() {
    let genotype = MultiDiscreteGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0],
            vec![0, 1, 2, 3],
        ])
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumMultiDiscreteGenotype)
        .with_reporter(PermutateReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = permutate.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(8));
    assert_eq!(inspect::chromosome(&best_chromosome), vec![4, 1, 0, 3]);
}

#[test]
fn call_multi_thread() {
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(5)
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();

    let mut rng = rand::thread_rng();
    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumDiscreteGenotype)
        .with_multithreading(true)
        // .with_reporter(PermutateReporterNoop::new())
        .call(&mut rng)
        .unwrap();

    let best_chromosome = permutate.best_chromosome().unwrap();
    println!("{:#?}", best_chromosome);

    assert_eq!(best_chromosome.fitness_score, Some(45));
    assert_eq!(inspect::chromosome(&best_chromosome), vec![9, 9, 9, 9, 9]);
}
