use genetic_algorithm::strategy::evolve::prelude::*;

#[derive(Clone, Debug)]
pub struct DistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DistanceTo {
    type Genotype = MatrixGenotype<f32, 100, 200>;
    fn call_for_population(
        &mut self,
        population: &mut Population<Self::Genotype>,
        genotype: &Self::Genotype,
        _thread_local: Option<&ThreadLocal<RefCell<Self>>>,
    ) {
        for chromosome in population.chromosomes.iter_mut() {
            let score = genotype
                .get_genes(chromosome.reference_id)
                .iter()
                .map(|v| (v - self.0).abs() / self.1)
                .sum::<f32>() as FitnessValue;
            chromosome.fitness_score = Some(score);
        }
    }
    fn calculate_for_chromosome(
        &mut self,
        _chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        None
    }
}

fn main() {
    env_logger::init();

    let genotype = MatrixGenotype::<f32, 100, 200>::builder()
        .with_genes_size(100)
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
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_target_fitness_score(100 * 100)
        .with_fitness(DistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverUniform::new())
        .with_select(SelectTournament::new(4, 0.9))
        .with_reporter(EvolveReporterSimple::new(100))
        .call()
        .unwrap();

    let duration = now.elapsed();

    println!("{}", evolve);
    if let Some(reference_id) = evolve.best_chromosome().map(|c| c.reference_id) {
        println!(
            "genes from store: {:?}",
            &evolve.genotype.get_genes(reference_id)
        );
    }
    println!("duration: {:?}", duration);
}
