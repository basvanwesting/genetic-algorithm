use genetic_algorithm::fitness::FitnessCountTrue;
use genetic_algorithm::permutate::prelude::*;

fn main() {
    let genotype = BinaryGenotype::builder()
        .with_gene_size(16)
        .build()
        .unwrap();

    println!("{}", genotype);

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(FitnessCountTrue)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .build()
        .unwrap()
        .call();

    println!("{}", permutate);
}
