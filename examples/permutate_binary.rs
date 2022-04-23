use genetic_algorithm::fitness;
use genetic_algorithm::genotype::BinaryRandomGenotype;
use genetic_algorithm::permutate::Permutate;

fn main() {
    let genotype = BinaryRandomGenotype::new().with_gene_size(16);

    println!("{}", genotype);

    let permutate = Permutate::new(genotype)
        .with_fitness(fitness::SimpleSum)
        .call();

    println!("{}", permutate);
}
