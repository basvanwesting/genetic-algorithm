use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 100;
const POPULATION_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct DistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DistanceTo {
    type Genotype = RangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .map(|v| (v - self.0).abs() / self.1)
                .sum::<f32>() as FitnessValue,
        )
    }
}

fn main() {
    env_logger::init();

    let genotype = RangeGenotype::<f32>::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0.0..=1.0) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.1..=0.1) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.001..=0.001) // slow converge
        .with_allele_mutation_scaled_range(vec![
            -0.1..=0.1,
            -0.01..=0.01,
            -0.001..=0.001,
            -0.0001..=0.0001,
            -0.00001..=0.0001,
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score(POPULATION_SIZE as isize * 100)
        .with_fitness(DistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateMultiGene::new(2, 0.2))
        .with_crossover(CrossoverMultiPoint::new(0.8, 0.05, 9, false))
        .with_select(SelectTournament::new(4, 0.4))
        .with_reporter(EvolveReporterSimple::new(100))
        .call()
        .unwrap();

    println!("{}", evolve);
}
