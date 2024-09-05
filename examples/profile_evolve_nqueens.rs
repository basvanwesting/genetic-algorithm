use criterion::*;
use pprof::criterion::*;

use genetic_algorithm::strategy::evolve::prelude::*;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueGenotype<u8>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
        _genotype: &Self::Genotype,
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
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..64).collect())
        .build()
        .unwrap();

    let evolve_builder = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_stale_generations(10000)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_target_fitness_score(0)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_fitness(NQueensFitness)
        .with_crossover(CrossoverClone::new())
        .with_select(SelectElite::new(0.9));

    c.bench_function("profile_evolve_nqueens", |b| {
        b.iter_batched(
            || evolve_builder.clone().build().unwrap(),
            |mut e| e.call(),
            BatchSize::SmallInput,
        );
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
}

criterion_main!(benches);
