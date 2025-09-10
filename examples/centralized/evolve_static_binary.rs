use genetic_algorithm::centralized::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 100;
const POPULATION_SIZE: usize = 100;
const SELECTION_RATE: f32 = 0.7;
const MATRIX_POP_SIZE: usize =
    POPULATION_SIZE + (POPULATION_SIZE as f32 * SELECTION_RATE + 1.0) as usize;

// Count the number of true values in the chromosome
#[derive(Clone, Debug)]
pub struct CountOnes;
impl Fitness for CountOnes {
    type Genotype = StaticBinaryGenotype<GENES_SIZE, MATRIX_POP_SIZE>;

    fn calculate_for_population(
        &mut self,
        _population: &Population,
        genotype: &FitnessGenotype<Self>,
    ) -> Vec<Option<FitnessValue>> {
        // pure matrix data calculation on [[T; N] M]
        // the order of the rows needs to be preserved as it matches the row_id on the chromosome
        genotype
            .data
            .iter()
            .map(|genes| genes.iter().filter(|&value| *value).count() as FitnessValue)
            .map(Some)
            .collect()
    }
}

fn main() {
    env_logger::init();

    let genotype = StaticBinaryGenotype::<GENES_SIZE, MATRIX_POP_SIZE>::builder()
        .with_genes_size(GENES_SIZE)
        .build()
        .unwrap();

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        // .with_select(SelectElite::new(0.5, 0.5))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_crossover(CrossoverUniform::new(SELECTION_RATE, 0.5))
        .with_mutate(MutateMultiGene::new(3, 0.01))
        .with_fitness(CountOnes)
        .with_fitness_ordering(FitnessOrdering::Maximize)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_reporter(EvolveReporterSimple::new(10))
        .build()
        .unwrap();

    evolve.call();

    let (best_genes, best_fitness) = evolve.best_genes_and_fitness_score().unwrap();

    println!("\nBest solution found:");
    println!("Number of true values: {}", best_fitness);
    println!("First 20 genes: {:?}", &best_genes[..20]);
}
