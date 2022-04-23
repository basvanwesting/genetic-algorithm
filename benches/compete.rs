use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use genetic_algorithm::compete;
use genetic_algorithm::compete::Compete;
use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::fitness;
use genetic_algorithm::fitness::Fitness;

pub fn criterion_benchmark(c: &mut Criterion) {
    let gen_genotype = Genotype::new()
        .with_gene_size(11)
        .with_gene_values(vec![true, false]);
    let population = gen_genotype.permutation_population_factory();
    let population = fitness::SimpleSum.call_for_population(population);

    let genotype = Genotype::new().with_population_size(1024);

    println!(
        "start population size: {}, target population size: {}",
        population.size(),
        genotype.population_size,
    );

    c.bench_function("compete_tournament", |b| {
        let compete = compete::Tournament(4);
        b.iter_batched(
            || population.clone(),
            |data| compete.call(&genotype, data),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("compete_elite", |b| {
        let compete = compete::Elite;
        b.iter_batched(
            || population.clone(),
            |data| compete.call(&genotype, data),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
