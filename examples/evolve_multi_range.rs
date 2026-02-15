//! Heterogeneous optimization using Evolve strategy with MultiRangeGenotype.
//! Each gene has its own range and mutation type, enabling mixed parameter optimization.
//! This example optimizes 4 heterogeneous parameters: a boolean flag, a categorical choice,
//! a continuous value, and a discrete integer.
use genetic_algorithm::strategy::evolve::prelude::*;

const POPULATION_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct HeterogeneousFitness {
    precision: f32,
}

impl Fitness for HeterogeneousFitness {
    type Genotype = MultiRangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        // Gene 0: boolean flag (0.0 or 1.0) - want enabled (1.0)
        let flag = chromosome.genes[0];
        // Gene 1: algorithm choice (0,1,2,3,4) - want algorithm 3
        let algorithm = chromosome.genes[1];
        // Gene 2: learning rate (0.001 to 1.0) - want 0.01
        let learning_rate = chromosome.genes[2];
        // Gene 3: batch size (16 to 512) - want 64
        let batch_size = chromosome.genes[3];

        let score = (flag - 1.0).abs()
            + (algorithm - 3.0).abs()
            + (learning_rate - 0.01).abs() * 100.0
            + (batch_size - 64.0).abs() / 100.0;

        Some((score / self.precision) as FitnessValue)
    }
}

fn main() {
    env_logger::init();

    let genotype = MultiRangeGenotype::<f32>::builder()
        .with_allele_ranges(vec![
            0.0..=1.0,     // Gene 0: boolean flag
            0.0..=4.0,     // Gene 1: algorithm choice (5 options)
            0.001..=1.0,   // Gene 2: learning rate (continuous)
            16.0..=512.0,  // Gene 3: batch size (discrete)
        ])
        .with_mutation_types(vec![
            MutationType::Discrete,                             // boolean: 0 or 1
            MutationType::Discrete,                             // enum: 0,1,2,3,4
            MutationType::StepScaled(vec![0.1, 0.01, 0.001]),  // continuous refinement
            MutationType::Discrete,                             // integer steps
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(1000)
        .with_target_fitness_score(0)
        .with_fitness(HeterogeneousFitness { precision: 1e-3 })
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_reporter(EvolveReporterSimple::new(100))
        .call()
        .unwrap();

    println!("{}", evolve);

    if let Some((best_genes, best_fitness_score)) = evolve.best_genes_and_fitness_score() {
        println!("flag: {}", best_genes[0]);
        println!("algorithm: {}", best_genes[1]);
        println!("learning_rate: {}", best_genes[2]);
        println!("batch_size: {}", best_genes[3]);
        println!("fitness: {}", best_fitness_score);
    }
}
