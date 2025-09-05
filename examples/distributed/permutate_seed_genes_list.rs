use genetic_algorithm::distributed::fitness::placeholders::CountOnes;
use genetic_algorithm::distributed::strategy::permutate::prelude::*;

fn main() {
    env_logger::init();

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

    println!("{}", genotype);

    let mut permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountOnes)
        .with_reporter(PermutateReporterSimple::new(usize::MAX))
        .build()
        .unwrap();

    permutate.call();
    println!();
    println!("{}", permutate);
}
