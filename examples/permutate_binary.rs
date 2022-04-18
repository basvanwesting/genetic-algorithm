use genetic_algorithm::context::Context;
use genetic_algorithm::fitness;
use genetic_algorithm::permutate::Permutate;

fn main() {
    let context = Context::new()
        .with_gene_size(16)
        .with_gene_values(vec![true, false]);

    println!("{}", context);

    let permutate = Permutate::new(context)
        .with_fitness(fitness::SimpleSum)
        .call();

    println!("{}", permutate);
}
