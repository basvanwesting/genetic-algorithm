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
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let mut score = 0;
        let genes_size = chromosome.genes.len();
        for i in 0..genes_size {
            for j in 0..genes_size {
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
        // .with_crossover(CrossoverRejuvenate::new(0.9)) // works remarkably bad, extremely interesting why, probably the best_genes are lost all the time?
        .with_crossover(CrossoverClone::new(0.9))
        .with_select(SelectElite::new(0.5, 0.5));

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
