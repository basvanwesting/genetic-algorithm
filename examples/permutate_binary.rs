use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::permutate::prelude::*;

fn main() {
    let genotype = BinaryGenotype::builder()
        .with_gene_size(16)
        .build()
        .unwrap();

    println!("{}", genotype);

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(CountTrue)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .call()
        .unwrap();

    println!("{}", permutate);
}
