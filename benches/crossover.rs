use criterion::*;
use genetic_algorithm::crossover::*;
use genetic_algorithm::genotype::{BinaryGenotype, Genotype};
use genetic_algorithm::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn setup(
    genes_size: usize,
    population_size: usize,
    rng: &mut SmallRng,
) -> (BinaryGenotype, Population<BinaryGenotype>) {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(genes_size)
        .build()
        .unwrap();

    let chromosomes = (0..population_size)
        .map(|_| genotype.chromosome_factory(rng))
        .collect();

    let population = Population::new(chromosomes);
    (genotype, population)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = SmallRng::from_entropy();
    let population_size: usize = 1000;
    let genes_sizes = vec![10, 100, 1000, 10000];

    let crossovers = vec![
        CrossoverDispatch(Crossovers::SingleGene, true),
        CrossoverDispatch(Crossovers::SingleGene, false),
        CrossoverDispatch(Crossovers::Uniform, true),
        CrossoverDispatch(Crossovers::Uniform, false),
        CrossoverDispatch(Crossovers::SinglePoint, true),
        CrossoverDispatch(Crossovers::SinglePoint, false),
        CrossoverDispatch(Crossovers::Clone, true),
        //CrossoverDispatch(Crossovers::Clone, false), //noop
    ];

    let mut group = c.benchmark_group(format!("crossovers-pop{}", population_size));
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for crossover in crossovers {
        for genes_size in &genes_sizes {
            group.throughput(Throughput::Elements(population_size as u64));
            let (genotype, population) = setup(*genes_size, population_size, &mut rng);
            group.bench_with_input(
                BenchmarkId::new(format!("{:?}-{}", crossover.0, crossover.1), genes_size),
                genes_size,
                |b, &_genes_size| {
                    b.iter_batched(
                        || population.clone(),
                        |mut data| crossover.call(&genotype, &mut data, &mut rng),
                        BatchSize::SmallInput,
                    )
                },
            );
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
