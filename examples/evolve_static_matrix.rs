use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 100;
const POPULATION_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct DistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DistanceTo {
    type Genotype = StaticMatrixGenotype<f32, GENES_SIZE, { POPULATION_SIZE + 2 }>;
    fn call_for_population(
        &mut self,
        population: &mut Population<StaticMatrixChromosome>,
        genotype: &mut FitnessGenotype<Self>,
        _thread_local: Option<&ThreadLocal<RefCell<Self>>>,
    ) {
        // pure matrix data calculation on [[T; N] M]
        let results: Vec<FitnessValue> = genotype
            .data
            .iter()
            .map(|genes| {
                genes
                    .iter()
                    .map(|v| (v - self.0).abs() / self.1)
                    .sum::<f32>() as FitnessValue
            })
            .collect();

        // result assignment
        for chromosome in population.chromosomes.iter_mut() {
            chromosome.fitness_score = Some(results[chromosome.reference_id]);
        }
    }
}

fn main() {
    env_logger::init();

    let genotype = StaticMatrixGenotype::<f32, GENES_SIZE, { POPULATION_SIZE + 2 }>::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0.0..=1.0) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.1..=0.1) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.001..=0.001) // slow converge
        .with_allele_mutation_scaled_range(vec![
            -0.1..=0.1,
            -0.05..=0.05,
            -0.025..=0.025,
            -0.01..=0.01,
            -0.005..=0.005,
            -0.0025..=0.0025,
            -0.001..=0.001,
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let now = std::time::Instant::now();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(DistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateMultiGene::new(2, 0.2))
        .with_crossover(CrossoverMultiPoint::new(9, false))
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(EvolveReporterSimple::new(100))
        .call()
        .unwrap();

    let duration = now.elapsed();

    println!("{}", evolve);
    println!("genes: {:?}", evolve.best_genes());
    println!("duration: {:?}", duration);
}
