#[cfg(test)]
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::strategy::permutate::prelude::*;

//#[test]
//build_invalid cannot be tested because invalid doesn't even have a type

#[test]
fn call_binary_maximize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(5));
    assert_eq!(
        permutate.best_genes().unwrap(),
        vec![true, true, true, true, true]
    );
}

#[test]
fn call_binary_minimize() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(5)
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_fitness(CountTrue)
        // .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(0));
    assert_eq!(
        permutate.best_genes().unwrap(),
        vec![false, false, false, false, false]
    );
}

#[test]
fn call_list() {
    let genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new())
        .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(45));
    assert_eq!(permutate.best_genes().unwrap(), vec![9, 9, 9, 9, 9]);
}

#[test]
fn call_multi_list() {
    let genotype = MultiListGenotype::builder()
        .with_allele_lists(vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1],
            vec![0],
            vec![0, 1, 2, 3],
        ])
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new())
        // .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(8));
    assert_eq!(permutate.best_genes().unwrap(), vec![4, 1, 0, 3]);
}

#[test]
fn call_par_fitness() {
    let genotype = ListGenotype::builder()
        .with_genes_size(5)
        .with_allele_list((0..10).collect())
        .build()
        .unwrap();

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(SumGenes::new())
        .with_par_fitness(true)
        .with_reporter(StrategyReporterNoop::new())
        .call()
        .unwrap();

    println!("{:#?}", permutate.best_genes());
    assert_eq!(permutate.best_fitness_score(), Some(45));
    assert_eq!(permutate.best_genes().unwrap(), vec![9, 9, 9, 9, 9]);
}
