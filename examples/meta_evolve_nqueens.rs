use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::evolve_stats::EvolveStats;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::{MultiIndexGenotype, UniqueIndexGenotype};
use genetic_algorithm::mutate;
use genetic_algorithm::permutate::Permutate;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::time::Instant;

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

#[derive(Clone, Debug)]
struct MetaFitness {
    rounds: usize,
    population_sizes: Vec<usize>,
    mutation_probabilities: Vec<f32>,
}
impl Fitness for MetaFitness {
    type Genotype = MultiIndexGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        let population_size = self.population_sizes[chromosome.genes[0]];
        let mutation_probability = self.mutation_probabilities[chromosome.genes[1]];

        let mut stats = EvolveStats::new();
        for _ in 0..self.rounds {
            let rng = SmallRng::from_entropy();
            let genotype = UniqueIndexGenotype::new().with_gene_size(32).build();
            let now = Instant::now();

            let evolve = Evolve::new(genotype, rng)
                .with_population_size(population_size)
                .with_max_stale_generations(1000)
                .with_target_fitness_score(0)
                .with_mutate(mutate::SingleGene(mutation_probability))
                .with_fitness(NQueensFitness)
                .with_crossover(crossover::Cloning(true))
                .with_compete(compete::Tournament(4))
                .call();

            stats.durations.push(now.elapsed());
            stats.best_generations.push(evolve.best_generation);
            stats.best_fitness_scores.push(evolve.best_fitness_score());
        }
        println!(
            "population_size: {}, mutation_probability: {:.*} | {}",
            population_size, 2, mutation_probability, stats
        );

        let mut score: isize = 0;
        if stats.best_fitness_score_mean() == 0.0 {
        } else {
            score -= 10_000
        }
        score -= stats.duration_mean_subsec_millis() as isize;
        score
    }
}

fn main() {
    let population_sizes = vec![10, 20, 50, 100, 200];
    let mutation_probabilities = vec![0.05, 0.1, 0.2, 0.3, 0.4, 0.5];

    let fitness = MetaFitness {
        rounds: 10,
        population_sizes: population_sizes.clone(),
        mutation_probabilities: mutation_probabilities.clone(),
    };

    //let rng = SmallRng::from_entropy();
    let genotype = MultiIndexGenotype::new()
        .with_gene_value_sizes(vec![population_sizes.len(), mutation_probabilities.len()])
        .build();

    println!("{}", genotype);

    let permutate = Permutate::new(genotype).with_fitness(fitness).call();

    println!();
    println!("{}", permutate);

    if let Some(best_chromosome) = permutate.best_chromosome {
        println!("best chromosome:");
        println!(
            "  population_size: {}",
            population_sizes[best_chromosome.genes[0]]
        );
        println!(
            "  mutation_probability: {}",
            mutation_probabilities[best_chromosome.genes[1]]
        );
    }
}
