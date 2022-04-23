use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::fitness;
use genetic_algorithm::permutate::Permutate;

fn main() {
    let genotype = Genotype::new()
        .with_gene_size(16)
        .with_gene_values(vec![true, false]);

    println!("{}", genotype);

    let permutate = Permutate::new(genotype)
        .with_fitness(fitness::SimpleSum)
        .call();

    println!("{}", permutate);
}
