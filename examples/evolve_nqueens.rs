use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::gene::DiscreteGene;
use genetic_algorithm::genotype::DiscreteUniqueGenotype;
use genetic_algorithm::mutate;
use rand::prelude::*;
use rand::rngs::SmallRng;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness<DiscreteGene> for NQueensFitness {
    fn call_for_chromosome(&self, chromosome: &Chromosome<DiscreteGene>) -> isize {
        let mut score = chromosome.genes.len() as isize;
        let max_index = chromosome.genes.len() - 1;
        for i in 0..max_index {
            for j in 0..max_index {
                if i != j {
                    let dx = i.abs_diff(j);
                    let dy = chromosome.genes[i].abs_diff(chromosome.genes[j]) as usize;
                    if dx == dy {
                        //diagonal clash
                        score -= 1;
                    }
                }
            }
        }
        score
    }
}

fn main() {
    let rng = SmallRng::from_entropy();
    let genotype = DiscreteUniqueGenotype::new().with_gene_values(vec![0, 1, 2, 3, 4, 5, 6, 7]);

    println!("{}", genotype);

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(20)
        .with_max_stale_generations(100)
        .with_target_fitness_score(8)
        .with_mutate(mutate::SingleGene(0.2))
        .with_fitness(NQueensFitness)
        .with_crossover(crossover::Cloning(true))
        .with_compete(compete::Elite)
        .call();

    println!("{}", evolve);
}
