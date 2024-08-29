use criterion::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::Rng;
use rayon::prelude::*;
// use thread_local::ThreadLocal;
use itertools::Itertools;

pub fn rng_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rng-benchmark");

    const CHUNK_SIZE: usize = 1000;
    const VEC_SIZE: usize = 100 * CHUNK_SIZE;

    group.bench_function("seq_thread_rng", |b| {
        let rng = &mut rand::thread_rng();
        b.iter(|| {
            let mut v = vec![0u8; VEC_SIZE];
            v.chunks_mut(CHUNK_SIZE).for_each(|chunk| rng.fill(chunk));
        });
    });

    // group.bench_function("seq_thread_rng_instatiate", |b| {
    //     b.iter(|| {
    //         let rng = &mut rand::thread_rng();
    //         let mut v = vec![0u8; VEC_SIZE];
    //         v.chunks_mut(CHUNK_SIZE).for_each(|chunk| rng.fill(chunk));
    //     });
    // });

    group.bench_function("par_thread_rng", |b| {
        b.iter(|| {
            let mut v = vec![0u8; VEC_SIZE];
            v.par_chunks_mut(CHUNK_SIZE)
                .for_each_init(rand::thread_rng, |rng, chunk| rng.fill(chunk));
        });
    });

    group.bench_function("seq_small_rng", |b| {
        let rng = &mut SmallRng::from_entropy();
        b.iter(|| {
            let mut v = vec![0u8; VEC_SIZE];
            v.chunks_mut(CHUNK_SIZE).for_each(|chunk| rng.fill(chunk));
        });
    });

    // group.bench_function("seq_small_rng_instatiate", |b| {
    //     b.iter(|| {
    //         let rng = &mut SmallRng::from_entropy();
    //         let mut v = vec![0u8; VEC_SIZE];
    //         v.chunks_mut(CHUNK_SIZE).for_each(|chunk| rng.fill(chunk));
    //     });
    // });

    // // invalid yields identical rngs
    // group.bench_function("par_small_rng_clone", |b| {
    //     let rng = SmallRng::from_entropy();
    //     b.iter(|| {
    //         let mut v = vec![0u8; VEC_SIZE];
    //         v.par_chunks_mut(CHUNK_SIZE)
    //             .for_each_with(rng.clone(), |rng, chunk| rng.fill(chunk));
    //     });
    // });

    // doesn't compile as the init is Fn and not FnMut
    // group.bench_function("par_small_rng_from_seed", |b| {
    //     let rng = &mut SmallRng::from_entropy();
    //     b.iter(|| {
    //         let mut v = vec![0u8; VEC_SIZE];
    //         v.par_chunks_mut(CHUNK_SIZE).for_each_init(
    //             || SmallRng::seed_from_u64(rng.gen()),
    //             |rng, chunk| rng.fill(chunk),
    //         );
    //     });
    // });

    group.bench_function("par_small_rng_from_thread_rng", |b| {
        b.iter(|| {
            let mut v = vec![0u8; VEC_SIZE];
            v.par_chunks_mut(CHUNK_SIZE).for_each_init(
                || SmallRng::from_rng(rand::thread_rng()).unwrap(),
                |rng, chunk| rng.fill(chunk),
            );
        });
    });

    // group.bench_function("par_small_rng_from_entropy", |b| {
    //     b.iter(|| {
    //         let mut v = vec![0u8; VEC_SIZE];
    //         v.par_chunks_mut(CHUNK_SIZE)
    //             .for_each_init(SmallRng::from_entropy, |rng, chunk| rng.fill(chunk));
    //     });
    // });
    //
    // group.bench_function("par_small_rng_thread_local", |b| {
    //     let thread_local = ThreadLocal::new();
    //     let rng = &mut SmallRng::from_entropy();
    //     b.iter(|| {
    //         let mut v = vec![0u8; VEC_SIZE];
    //         v.par_chunks_mut(CHUNK_SIZE).for_each_init(
    //             || {
    //                 thread_local
    //                     .get_or(|| std::cell::RefCell::new(rng.clone()))
    //                     .borrow_mut()
    //             },
    //             |rng, chunk| rng.fill(chunk),
    //         );
    //     });
    // });
}

// struct MyIndexSampler<'a> {
//     indexes: Vec<usize>,
//     rng: &'a mut SmallRng,
// }
// impl<'a> MyIndexSampler<'a> {
//     pub fn new(indexes: Vec<usize>, rng: &'a mut SmallRng) -> Self {
//         Self { indexes, rng }
//     }
// }
// impl<'a> Iterator for MyIndexSampler<'a> {
//     type Item = usize;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.indexes.pop()
//     }
// }

pub fn seq_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("seq-benchmark");

    let data = vec![
        // (5, 10),
        // (10, 100),
        (50, 100),
        // (10, 1000),
        (100, 1000),
        // (10, 10000),
        // (100, 10000),
    ];
    let rng = &mut SmallRng::from_entropy();

    for (amount, length) in data {
        group.bench_function(
            format!("rand::seq::index::sample-{}-{}", amount, length),
            |b| {
                b.iter_batched(
                    || ((0..length).collect::<Vec<_>>(), Vec::with_capacity(amount)),
                    |(_source, mut target)| {
                        rand::seq::index::sample(rng, length, amount)
                            .iter()
                            .for_each(|x| target.push(x));
                    },
                    BatchSize::SmallInput,
                );
            },
        );
        group.bench_function(
            format!("rand::seq::index::sample-sort-{}-{}", amount, length),
            |b| {
                b.iter_batched(
                    || ((0..length).collect::<Vec<_>>(), Vec::with_capacity(amount)),
                    |(_source, mut target)| {
                        rand::seq::index::sample(rng, length, amount)
                            .iter()
                            .sorted_unstable()
                            .for_each(|x| target.push(x));
                    },
                    BatchSize::SmallInput,
                );
            },
        );
        group.bench_function(
            format!("rand::seq::index::sample-sort-dedup{}-{}", amount, length),
            |b| {
                b.iter_batched(
                    || ((0..length).collect::<Vec<_>>(), Vec::with_capacity(amount)),
                    |(_source, mut target)| {
                        rand::seq::index::sample(rng, length, amount)
                            .iter()
                            .sorted_unstable()
                            .dedup()
                            .for_each(|x| target.push(x));
                    },
                    BatchSize::SmallInput,
                );
            },
        );
        group.bench_function(
            format!("rand::seq::index::sample-unique{}-{}", amount, length),
            |b| {
                b.iter_batched(
                    || ((0..length).collect::<Vec<_>>(), Vec::with_capacity(amount)),
                    |(_source, mut target)| {
                        rand::seq::index::sample(rng, length, amount)
                            .iter()
                            .unique()
                            .for_each(|x| target.push(x));
                    },
                    BatchSize::SmallInput,
                );
            },
        );
        // group.bench_function(format!("MyIndexSampler-{}-{}", amount, length), |b| {
        //     b.iter_batched(
        //         || ((0..length).collect::<Vec<_>>(), Vec::with_capacity(amount)),
        //         |(source, mut target)| {
        //             MyIndexSampler::new(source.clone(), rng)
        //                 .take(amount)
        //                 .for_each(|x| target.push(x));
        //         },
        //         BatchSize::SmallInput,
        //     );
        // });
    }
}

criterion_group!(benches, seq_benchmark, rng_benchmark);
criterion_main!(benches);
