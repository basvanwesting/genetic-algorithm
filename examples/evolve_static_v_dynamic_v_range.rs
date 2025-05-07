use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 1000;
const POPULATION_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct StaticDistanceTo(pub f32, pub f32); // target, precision
impl Fitness for StaticDistanceTo {
    type Genotype = StaticMatrixGenotype<f32, GENES_SIZE, POPULATION_SIZE>;
    fn calculate_for_population(
        &mut self,
        _population: &Population<StaticMatrixChromosome>,
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

#[derive(Clone, Debug)]
pub struct DynamicDistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DynamicDistanceTo {
    type Genotype = DynamicMatrixGenotype;
    fn calculate_for_population(
        &mut self,
        _population: &Population<DynamicMatrixChromosome>,
        genotype: &FitnessGenotype<Self>,
    ) -> Vec<Option<FitnessValue>> {
        // pure matrix data calculation on vec![T; N*M]
        // the order of the rows needs to be preserved as it matches the row_id on the chromosome
        genotype
            .data
            .chunks(GENES_SIZE)
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

#[derive(Clone, Debug)]
pub struct RangeDistanceTo(pub f32, pub f32); // target, precision
impl Fitness for RangeDistanceTo {
    type Genotype = RangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &Self::Genotype,
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

    let genotype = StaticMatrixGenotype::<f32, GENES_SIZE, POPULATION_SIZE>::builder()
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
    // println!("{}", genotype);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(StaticDistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        // .with_crossover(CrossoverMultiPoint::new(0.4, 0.8, 10, true))
        .with_crossover(CrossoverUniform::new(0.4, 0.8))
        .with_select(SelectTournament::new(0.02, 4))
        .with_reporter(EvolveReporterDuration::new())
        .call()
        .unwrap();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_genes().unwrap());
    println!(
        "StaticMatrixGenotype, best_generation: {:?}",
        evolve.best_generation()
    );
    println!(
        "StaticMatrixGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );

    let genotype = DynamicMatrixGenotype::builder()
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
    // println!("{}", genotype);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(DynamicDistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverUniform::new(0.4, 0.8))
        .with_select(SelectTournament::new(0.02, 4))
        .with_reporter(EvolveReporterDuration::new())
        .call()
        .unwrap();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_genes().unwrap());
    println!(
        "DynamicMatrixGenotype, best_generation: {:?}",
        evolve.best_generation()
    );
    println!(
        "DynamicMatrixGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );

    let genotype = RangeGenotype::builder()
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
    // println!("{}", genotype);
    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(RangeDistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverUniform::new(0.4, 0.8))
        .with_select(SelectTournament::new(0.02, 4))
        .with_reporter(EvolveReporterDuration::new())
        .call()
        .unwrap();
    // println!("{}", evolve);
    // println!("genes: {:b}", evolve.best_genes().unwrap());
    println!(
        "RangeGenotype, best_generation: {:?}",
        evolve.best_generation()
    );
    println!(
        "RangeGenotype, best_fitness_score: {:?}",
        evolve.best_fitness_score()
    );
}
