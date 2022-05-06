use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::CompeteElite;
use genetic_algorithm::crossover::CrossoverClone;
use genetic_algorithm::evolve::Evolve;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering, FitnessValue};
use genetic_algorithm::genotype::UniqueIndexGenotype;
use genetic_algorithm::mutate::MutateOnce;
use rand::prelude::*;
use rand::rngs::SmallRng;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueIndexGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let mut score = 0;
        let max_index = chromosome.genes.len() - 1;
        for i in 0..max_index {
            for j in 0..max_index {
                if i != j {
                    let dx = i.abs_diff(j);
                    let dy = chromosome.genes[i].abs_diff(chromosome.genes[j]) as usize;
                    if dx == dy {
                        //diagonal clash
                        score += 1;
                    }
                }
            }
        }
        Some(score)
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("profile_evolve_nqueens", |b| b.iter(|| run()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}
criterion_main!(benches);

fn run() {
    let rng = SmallRng::from_entropy();
    let genotype = UniqueIndexGenotype::new().with_gene_value_size(64).build();

    let evolve = Evolve::new(genotype, rng)
        .with_population_size(20)
        .with_max_stale_generations(10000)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateOnce(0.2))
        .with_fitness(NQueensFitness)
        .with_crossover(CrossoverClone(true))
        .with_compete(CompeteElite)
        .call();

    println!("{}", evolve);
}