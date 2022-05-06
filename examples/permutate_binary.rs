use genetic_algorithm::fitness::{FitnessOrdering, FitnessSimpleCount};
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::permutate::Permutate;

fn main() {
    let genotype = BinaryGenotype::new().with_gene_size(16).build();

    println!("{}", genotype);

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(FitnessSimpleCount)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .build()
        .unwrap()
        .call();

    println!("{}", permutate);
}
