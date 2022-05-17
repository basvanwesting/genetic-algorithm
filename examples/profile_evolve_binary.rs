use genetic_algorithm::evolve::prelude::*;
use genetic_algorithm::fitness::placeholders::CountTrue;
use rand::prelude::*;
use rand::rngs::SmallRng;

fn main() {
    let mut rng = SmallRng::from_entropy();
    let genotype = BinaryGenotype::builder()
        .with_genes_size(1000)
        .build()
        .unwrap();

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_population_size(1000)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(0)
        .with_fitness(CountTrue)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateOnce(0.2))
        .with_crossover(CrossoverSinglePoint(true))
        .with_compete(CompeteTournament(4))
        .build()
        .unwrap();

    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    evolve.call(&mut rng);
    println!("{}", evolve);

    if let Ok(report) = guard.report().build() {
        let file = std::fs::File::create("flamegraph_binary.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}
