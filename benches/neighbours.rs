use criterion::*;
use genetic_algorithm::genotype::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
//use std::time::Duration;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("neighbours");
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    let mut rng = SmallRng::from_entropy();

    group.bench_function("mult-continues-chromosome_neighbours", |b| {
        let genotype = MultiContinuousGenotype::builder()
            .with_allele_multi_range(vec![(-100.0..100.0), (0.0..100.0)])
            .with_allele_multi_neighbour_range(vec![(-1.0..1.0), (-1.0..1.0)])
            .build()
            .unwrap();

        b.iter_batched(
            || genotype.chromosome_factory(&mut rng),
            |c| genotype.chromosome_neighbours(&c, Some(1.0)),
            BatchSize::SmallInput,
        );
    });

    group.bench_function("mult-continues-chromosome_neighbour_permutations", |b| {
        let genotype = MultiContinuousGenotype::builder()
            .with_allele_multi_range(vec![(-100.0..100.0), (0.0..100.0)])
            .with_allele_multi_neighbour_range(vec![(-1.0..1.0), (-1.0..1.0)])
            .build()
            .unwrap();

        b.iter_batched(
            || genotype.chromosome_factory(&mut rng),
            |c| genotype.chromosome_neighbour_permutations(&c, Some(1.0)),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
