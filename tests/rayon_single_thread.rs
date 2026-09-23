// Separate test binary, as the rayon global thread pool can only be configured once per process.
// Checks the parallel strategy calls do not deadlock on a single rayon thread (e.g. a 1 vCPU
// container or CI runner).
use genetic_algorithm::fitness::placeholders::{CountTrue, SumGenes};
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm::strategy::hill_climb::prelude::*;
use genetic_algorithm::strategy::permutate::prelude::*;
use std::sync::mpsc::channel;
use std::sync::Once;
use std::thread;
use std::time::Duration;

static INIT: Once = Once::new();

fn single_rayon_thread() {
    INIT.call_once(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .unwrap();
    });
}

// fail instead of hanging the test run on a deadlock
fn with_timeout<F: FnOnce() + Send + 'static>(f: F) {
    single_rayon_thread();
    let (sender, receiver) = channel();
    thread::spawn(move || {
        f();
        sender.send(()).unwrap();
    });
    receiver
        .recv_timeout(Duration::from_secs(60))
        .expect("deadlock on a single rayon thread");
}

macro_rules! evolve_builder {
    () => {
        Evolve::builder()
            .with_genotype(
                BinaryGenotype::builder()
                    .with_genes_size(10)
                    .build()
                    .unwrap(),
            )
            .with_target_population_size(20)
            .with_max_stale_generations(10)
            .with_mutate(MutateSingleGene::new(0.1))
            .with_fitness(CountTrue)
            .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
            .with_select(SelectTournament::new(0.5, 0.02, 4))
            .with_reporter(StrategyReporterNoop::new())
    };
}

#[test]
fn evolve_call_par_repeatedly() {
    with_timeout(|| {
        let (best_run, _) = evolve_builder!().call_par_repeatedly(3).unwrap();
        assert!(best_run.best_fitness_score().is_some());
    });
}

#[test]
fn evolve_call_par_speciated() {
    with_timeout(|| {
        let (best_run, _) = evolve_builder!().call_par_speciated(3).unwrap();
        assert!(best_run.best_fitness_score().is_some());
    });
}

#[test]
fn hill_climb_call_par_repeatedly() {
    with_timeout(|| {
        let genotype = RangeGenotype::builder()
            .with_genes_size(5)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        let (best_run, _) = HillClimb::builder()
            .with_genotype(genotype)
            .with_max_stale_generations(10)
            .with_fitness(SumGenes::new_with_precision(1e-3))
            .with_reporter(StrategyReporterNoop::new())
            .call_par_repeatedly(3)
            .unwrap();
        assert!(best_run.best_fitness_score().is_some());
    });
}

#[test]
fn permutate_par_fitness() {
    with_timeout(|| {
        let genotype = ListGenotype::builder()
            .with_genes_size(3)
            .with_allele_list((0..10).collect())
            .build()
            .unwrap();
        let permutate = Permutate::builder()
            .with_genotype(genotype)
            .with_fitness(SumGenes::new())
            .with_par_fitness(true)
            .with_reporter(StrategyReporterNoop::new())
            .call()
            .unwrap();
        assert_eq!(permutate.best_fitness_score(), Some(27));
    });
}
