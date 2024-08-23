use criterion::*;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::Rng;
use rayon::prelude::*;
use std::ops::DerefMut;
use thread_local::ThreadLocal;

pub fn rand_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("playground");

    group.bench_function("seq_thread_rng", |b| {
        let rng = &mut rand::thread_rng();
        b.iter(|| {
            let mut v = vec![0u8; 100_000];
            v.chunks_mut(1000).for_each(|chunk| rng.fill(chunk));
        });
    });

    group.bench_function("seq_thread_rng_reinit", |b| {
        b.iter(|| {
            let rng = &mut rand::thread_rng();
            let mut v = vec![0u8; 100_000];
            v.chunks_mut(1000).for_each(|chunk| rng.fill(chunk));
        });
    });

    group.bench_function("par_thread_rng", |b| {
        b.iter(|| {
            let mut v = vec![0u8; 100_000];
            v.par_chunks_mut(1000)
                .for_each_init(rand::thread_rng, |rng, chunk| rng.fill(chunk));
        });
    });

    group.bench_function("seq_small_rng", |b| {
        let rng = &mut SmallRng::from_entropy();
        b.iter(|| {
            let mut v = vec![0u8; 100_000];
            v.chunks_mut(1000).for_each(|chunk| rng.fill(chunk));
        });
    });

    group.bench_function("seq_small_rng_reinit", |b| {
        b.iter(|| {
            let rng = &mut SmallRng::from_entropy();
            let mut v = vec![0u8; 100_000];
            v.chunks_mut(1000).for_each(|chunk| rng.fill(chunk));
        });
    });

    group.bench_function("par_small_rng", |b| {
        b.iter(|| {
            let mut v = vec![0u8; 100_000];
            v.par_chunks_mut(1000)
                .for_each_init(SmallRng::from_entropy, |rng, chunk| rng.fill(chunk));
        });
    });

    group.bench_function("par_small_rng_thread_local", |b| {
        let thread_local = ThreadLocal::new();
        let rng = &mut SmallRng::from_entropy();
        b.iter(|| {
            let mut v = vec![0u8; 100_000];
            v.par_chunks_mut(1000).for_each_init(
                || {
                    thread_local
                        .get_or(|| std::cell::RefCell::new(rng.clone()))
                        .borrow_mut()
                },
                |rng, chunk| rng.fill(chunk),
            );
        });
    });

    group.bench_function("par_small_rng_thread_local_deref_mut", |b| {
        let thread_local = ThreadLocal::new();
        let rng = &mut SmallRng::from_entropy();
        b.iter(|| {
            let mut v = vec![0u8; 100_000];
            v.par_chunks_mut(1000).for_each_init(
                || {
                    thread_local
                        .get_or(|| std::cell::RefCell::new(rng.clone()))
                        .borrow_mut()
                },
                |rng, chunk| {
                    let local_rng = rng.deref_mut();
                    local_rng.fill(chunk)
                },
            );
        });
    });
}

criterion_group!(benches, rand_benchmark);
criterion_main!(benches);
