use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete;
use genetic_algorithm::crossover;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::RangeUniqueGenotype;
use genetic_algorithm::mutate;
use rand::prelude::*;
use rand::rngs::SmallRng;
use stats::{mean, stddev};
use std::time::{Duration, Instant};

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = RangeUniqueGenotype;
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

struct Data {
    pub durations: Vec<Duration>,
    pub best_generations: Vec<usize>,
    pub best_fitness_scores: Vec<Option<isize>>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            durations: vec![],
            best_generations: vec![],
            best_fitness_scores: vec![],
        }
    }

    pub fn duration_count(&self) -> usize {
        self.durations.len()
    }
    pub fn duration_stddev(&self) -> f64 {
        stddev(self.durations.iter().map(|c| c.subsec_millis()))
    }
    pub fn duration_mean(&self) -> f64 {
        mean(self.durations.iter().map(|c| c.subsec_millis()))
    }

    pub fn best_generation_count(&self) -> usize {
        self.best_generations.len()
    }
    pub fn best_generation_stddev(&self) -> f64 {
        stddev(self.best_generations.clone().into_iter())
    }
    pub fn best_generation_mean(&self) -> f64 {
        mean(self.best_generations.clone().into_iter())
    }

    pub fn best_fitness_score_count(&self) -> usize {
        self.best_fitness_scores
            .iter()
            .filter(|s| s.is_some())
            .count()
    }
    pub fn best_fitness_score_stddev(&self) -> f64 {
        stddev(self.best_fitness_scores.iter().filter_map(|s| *s))
    }
    pub fn best_fitness_score_mean(&self) -> f64 {
        mean(self.best_fitness_scores.iter().filter_map(|s| *s))
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "duration - count: {}, mean: {:.*}ms, stddev: {:.*}ms",
            self.duration_count(),
            1,
            self.duration_mean(),
            1,
            self.duration_stddev()
        )?;
        write!(f, " | ")?;
        write!(
            f,
            "best_generation - count: {}, mean: {:.*}, stddev: {:.*}",
            self.best_generation_count(),
            1,
            self.best_generation_mean(),
            1,
            self.best_generation_stddev()
        )?;
        write!(f, " | ")?;
        write!(
            f,
            "best_fitness_score - count: {}, mean: {:.*}, stddev: {:.*}",
            self.best_fitness_score_count(),
            1,
            self.best_fitness_score_mean(),
            1,
            self.best_fitness_score_stddev()
        )
    }
}

fn run_round(data: &mut Data, mutation_probability: f32) {
    let rng = SmallRng::from_entropy();
    let genotype = RangeUniqueGenotype::new().with_gene_range(0..32).build();
    let now = Instant::now();

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(100)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(0)
        .with_mutate(mutate::SingleGene(mutation_probability))
        .with_fitness(NQueensFitness)
        .with_crossover(crossover::Cloning(true))
        .with_compete(compete::Tournament(4))
        .call();

    data.durations.push(now.elapsed());
    data.best_generations.push(evolve.best_generation);
    data.best_fitness_scores.push(evolve.best_fitness_score());

    //println!("{}", evolve);
}

fn main() {
    let rounds = 100;

    for mutation_probability in vec![0.05, 0.1, 0.2, 0.3, 0.4, 0.5] {
        let mut data = Data::new();
        for _ in 0..rounds {
            run_round(&mut data, mutation_probability);
        }
        println!(
            "mutation_probability: {:.*} | {}",
            2, mutation_probability, data
        );
    }
}
