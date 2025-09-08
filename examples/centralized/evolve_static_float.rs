use genetic_algorithm::centralized::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 100;
const POPULATION_SIZE: usize = 100;
const SELECTION_RATE: f32 = 0.7;
const MATRIX_POP_SIZE: usize =
    POPULATION_SIZE + (POPULATION_SIZE as f32 * SELECTION_RATE + 1.0) as usize;

#[derive(Clone, Debug)]
pub struct DistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DistanceTo {
    type Genotype = StaticRangeGenotype<f32, GENES_SIZE, MATRIX_POP_SIZE>;
    fn calculate_for_population(
        &mut self,
        _population: &Population<StaticRangeChromosome>,
        genotype: &FitnessGenotype<Self>,
    ) -> Vec<Option<FitnessValue>> {
        // pure matrix data calculation on [[T; N] M]
        // the order of the rows needs to be preserved as it matches the row_id on the chromosome
        genotype
            .data
            .iter()
            .map(|genes| {
                genes
                    .iter()
                    .map(|v| (v - self.0).abs() / self.1)
                    .sum::<f32>() as FitnessValue
            })
            .map(Some)
            .collect()
    }
}

fn main() {
    env_logger::init();

    let genotype = StaticRangeGenotype::<f32, GENES_SIZE, MATRIX_POP_SIZE>::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0.0..=1.0) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.1..=0.1) // won't converge, with low max_stale_generations, converges just fine with higher max_stale_generations
        // .with_allele_mutation_range(-0.001..=0.001) // slow converge
        .with_allele_mutation_scaled_range(vec![
            -0.1..=0.1,
            -0.01..=0.01,
            -0.001..=0.001,
            -0.0001..=0.0001,
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(DistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateMultiGene::new(2, 0.2))
        .with_crossover(CrossoverMultiPoint::new(SELECTION_RATE, 0.8, 9, false))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_reporter(EvolveReporterSimple::new(100))
        .call()
        .unwrap();

    println!("{}", evolve);
    println!(
        "genes and fitness_score: {:?}",
        evolve.best_genes_and_fitness_score()
    );
}
