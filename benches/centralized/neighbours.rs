use criterion::*;
use genetic_algorithm::centralized::chromosome::ChromosomeManager;
use genetic_algorithm::centralized::genotype::*;
use genetic_algorithm::centralized::population::Population;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

const GENES_SIZE: usize = 10;
const MAX_POPULATION_SIZE: usize = 300;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("neighbouring_population");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let mut rng1 = SmallRng::from_entropy();
    let mut rng2 = SmallRng::from_entropy();

    group.bench_function("range-neighbouring_population-relative", |b| {
        let mut genotype = StaticRangeGenotype::<f32, GENES_SIZE, MAX_POPULATION_SIZE>::builder()
            .with_genes_size(GENES_SIZE)
            .with_allele_range(-1.0..=1.0)
            .with_allele_mutation_range(-0.1..=0.1)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        b.iter_batched(
            || {
                (
                    genotype.chromosome_constructor_random(&mut rng1),
                    genotype.clone(),
                    Population::new(vec![]),
                )
            },
            |(c, mut g, mut p)| g.fill_neighbouring_population(&c, &mut p, None, &mut rng2),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("range-neighbouring_population-scaled", |b| {
        let mut genotype = StaticRangeGenotype::<f32, GENES_SIZE, MAX_POPULATION_SIZE>::builder()
            .with_genes_size(GENES_SIZE)
            .with_allele_range(-1.0..=1.0)
            .with_allele_mutation_scaled_range(vec![-0.1..=0.1, -0.01..=0.01, -0.001..=0.001])
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        b.iter_batched(
            || {
                (
                    genotype.chromosome_constructor_random(&mut rng1),
                    genotype.clone(),
                    Population::new(vec![]),
                )
            },
            |(c, mut g, mut p)| g.fill_neighbouring_population(&c, &mut p, Some(1), &mut rng2),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
