use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::CompeteTournament;
use genetic_algorithm::crossover::CrossoverCloning;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::UniqueIndexGenotype;
use genetic_algorithm::mutate::MutateSingleGene;
use rand::prelude::*;
use rand::rngs::SmallRng;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueIndexGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        let mut score = 0;
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
    let genotype = UniqueIndexGenotype::new().with_gene_size(32).build();

    println!("{}", genotype);

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGene(0.2))
        .with_fitness(NQueensFitness)
        .with_crossover(CrossoverCloning(true))
        .with_compete(CompeteTournament(4))
        .call();

    println!("{}", evolve);
}
