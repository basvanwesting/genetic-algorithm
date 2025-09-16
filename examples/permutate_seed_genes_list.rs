use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::strategy::permutate::prelude::*;

fn main() {
    env_logger::init();

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

    println!("{}", genotype);

    let mut permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        .with_reporter(PermutateReporterSimple::new(usize::MAX))
        .build()
        .unwrap();

    permutate.call();
    println!();
    println!("{}", permutate);
}
