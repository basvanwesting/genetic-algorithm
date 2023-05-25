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

    let crossovers: Vec<CrossoverDispatch> = vec![
        CrossoverSingleGene::new(true).into(),
        CrossoverSingleGene::new(false).into(),
        CrossoverUniform::new(true).into(),
        CrossoverUniform::new(false).into(),
        CrossoverSinglePoint::new(true).into(),
        CrossoverSinglePoint::new(false).into(),
        CrossoverClone::new(true).into(),
        //CrossoverClone::new(false).into(), //noop
    ];

    let mut group = c.benchmark_group(format!("crossovers-pop{}", population_size));
    //group.warm_up_time(Duration::from_secs(3));
    //group.measurement_time(Duration::from_secs(3));
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for mut crossover in crossovers {
        for genes_size in &genes_sizes {
            group.throughput(Throughput::Elements(population_size as u64));
            let (genotype, population) = setup(*genes_size, population_size, &mut rng);
            group.bench_with_input(
                BenchmarkId::new(
                    format!("{:?}-{}", crossover.implementation, crossover.keep_parent),
                    genes_size,
                ),
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
