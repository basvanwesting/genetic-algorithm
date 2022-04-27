use genetic_algorithm::fitness;
use genetic_algorithm::genotype::BinaryGenotype;
use genetic_algorithm::permutate::Permutate;

fn main() {
    let genotype = BinaryGenotype::new().with_gene_size(16).build();

    println!("{}", genotype);

    let permutate = Permutate::new(genotype)
        .with_fitness(fitness::SimpleSum)
        .call();

    println!("{}", permutate);
}
