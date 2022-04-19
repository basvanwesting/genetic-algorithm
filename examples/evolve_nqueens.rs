use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete;
use genetic_algorithm::context::Context;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::gene::DiscreteGene;
use genetic_algorithm::mutate;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness<DiscreteGene> for NQueensFitness {
    fn call_for_chromosome(&self, chromosome: &Chromosome<DiscreteGene>) -> isize {
        let mut diagonal_clashes = 0;
        let max_index = chromosome.genes.len() - 1;
        for i in 0..max_index {
            for j in 0..max_index {
                if i != j {
                    let dx = i.abs_diff(j);
                    let dy = chromosome.genes[i].abs_diff(chromosome.genes[j]) as usize;
                    if dx == dy {
                        diagonal_clashes += 1;
                    }
                }
            }
        }
        let mut temp_genes = chromosome.genes.clone();
        temp_genes.sort();
        temp_genes.dedup();
        temp_genes.len() as isize - diagonal_clashes as isize
    }
}

fn main() {
    let context = Context::new()
        .with_gene_size(8)
        .with_gene_values(vec![0, 1, 2, 3, 4, 5, 6, 7])
        .with_population_size(10);

    println!("{}", context);

    let evolve = Evolve::new(context)
        .with_max_stale_generations(100)
        .with_target_fitness_score(8)
        .with_mutate(mutate::SwapSingleGene(0.2))
        .with_fitness(NQueensFitness)
        .with_crossover(crossover::Cloning)
        .with_compete(compete::Tournament(4))
        //.with_compete(compete::Elite)
        .call();

    println!("{}", evolve);
}
