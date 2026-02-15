//! Mixed-integer linear programming using Evolve with custom Mutate implementation.
//! Demonstrates how to implement custom mutation logic by implementing the Mutate trait.
use genetic_algorithm::crossover::Crossover;
use genetic_algorithm::extension::Extension;
use genetic_algorithm::mutate::Mutate;
use genetic_algorithm::select::Select;
use genetic_algorithm::strategy::evolve::prelude::*;
use genetic_algorithm::strategy::{StrategyAction, StrategyState};
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::Rng;
use std::time::Instant;

const TARGET_SCORE: isize = (59.0 / PRECISION) as isize;
const PRECISION: f32 = 1e-5;

#[derive(Clone, Debug)]
struct MILPFitness;
impl Fitness for MILPFitness {
    type Genotype = MultiRangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let x1 = chromosome.genes[0];
        let x2 = chromosome.genes[1].floor();

        if x1 + 2.0 * x2 >= -14.0 && -4.0 * x1 - x2 <= -33.0 && 2.0 * x1 + x2 <= 20.0 {
            let score = 8.0 * x1 + x2;
            Some((score / PRECISION) as isize)
        } else {
            None
        }
    }
}

/// Custom Mutate implementation that optionally mutates both genes diagonally at a 45-degree
/// angle. This demonstrates using the new associated type approach for custom mutations. Instead
/// of mutating X or Y independently, it applies mutations along a diagonal line, maintaining a
/// relationship between the two variables.
#[derive(Clone, Debug)]
struct ScaledOptionalDiagonalMutate {
    pub mutation_probability_sampler: Bernoulli,
    pub axis_probability_sampler: Uniform<usize>,
}

impl ScaledOptionalDiagonalMutate {
    pub fn new(mutation_probability: f32) -> Self {
        let mutation_probability_sampler = Bernoulli::new(mutation_probability as f64).unwrap();
        let axis_probability_sampler = Uniform::from(0..=3);
        Self {
            mutation_probability_sampler,
            axis_probability_sampler,
        }
    }
}

impl Mutate for ScaledOptionalDiagonalMutate {
    type Genotype = MultiRangeGenotype<f32>;

    fn call<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        for chromosome in state
            .population
            .chromosomes
            .iter_mut()
            .filter(|c| c.is_offspring())
        {
            if self.mutation_probability_sampler.sample(rng) {
                match self.axis_probability_sampler.sample(rng) {
                    0 => {
                        //25%
                        genotype.mutate_gene(chromosome, 0, rng);
                    }
                    1 => {
                        //25%
                        genotype.mutate_gene(chromosome, 1, rng);
                    }
                    _ => {
                        //50%
                        genotype.mutate_gene(chromosome, 0, rng);
                        genotype.mutate_gene(chromosome, 1, rng);
                    }
                }
                // remember to reset the chromosome metadata after manipulation
                chromosome.reset_metadata(genotype.genes_hashing);
            }
        }
        state.add_duration(StrategyAction::Mutate, now.elapsed());
    }
}

fn main() {
    env_logger::init();

    let genotype = MultiRangeGenotype::builder()
        .with_allele_ranges(vec![(-10.0..=10.0), (0.0..=10.0)])
        .with_mutation_types(vec![
            MutationType::StepScaled(vec![0.1, 0.01, 0.001, 0.0001, 0.00001, 0.000001]),
            MutationType::StepScaled(vec![0.5, 0.1, 0.05, 0.01, 0.001, 0.0001]),
        ])
        .build()
        .unwrap();

    println!("=== MILP with Custom Scaled Optional Diagonal Mutation Example ===\n");
    println!("This example demonstrates custom mutation strategies using associated types.");
    println!("We implement diagonal mutations that change both x1 and x2 simultaneously,");
    println!("exploring the search space along diagonal lines rather than only axis-aligned.\n");
    println!("=== Note ===");
    println!("It doesn't perform better, but that is not the point of the example.\n");

    println!("Genotype: {}\n", genotype);

    // Test with standard mutation for comparison
    println!("--- Standard Single Gene Mutation (baseline) ---");
    for run in 0..5 {
        let now = Instant::now();
        let evolve = Evolve::builder()
            .with_genotype(genotype.clone())
            .with_target_population_size(1000)
            .with_max_stale_generations(100)
            .with_target_fitness_score(TARGET_SCORE)
            .with_fitness_ordering(FitnessOrdering::Minimize)
            .with_mutate(MutateSingleGene::new(0.4))
            .with_fitness(MILPFitness)
            .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
            .with_select(SelectTournament::new(0.5, 0.02, 4))
            .call()
            .unwrap();
        let duration = now.elapsed();

        print_result(&evolve, "Standard", run + 1, duration);
    }

    // Test with diagonal mutation
    println!("\n--- Custom Scaled Optional Diagonal Mutation (45-degree) ---");
    for run in 0..5 {
        let now = Instant::now();
        let evolve = Evolve::builder()
            .with_genotype(genotype.clone())
            .with_target_population_size(1000)
            .with_max_stale_generations(100)
            .with_target_fitness_score(TARGET_SCORE)
            .with_fitness_ordering(FitnessOrdering::Minimize)
            .with_mutate(ScaledOptionalDiagonalMutate::new(0.4))
            .with_fitness(MILPFitness)
            .with_crossover(CrossoverSingleGene::new(0.7, 0.8))
            .with_select(SelectTournament::new(0.5, 0.02, 4))
            .call()
            .unwrap();
        let duration = now.elapsed();

        print_result(&evolve, "Diagonal", run + 1, duration);
    }
}

fn print_result(
    evolve: &Evolve<
        MultiRangeGenotype<f32>,
        impl Mutate<Genotype = MultiRangeGenotype<f32>>,
        MILPFitness,
        impl Crossover<Genotype = MultiRangeGenotype<f32>>,
        impl Select<Genotype = MultiRangeGenotype<f32>>,
        impl Extension<Genotype = MultiRangeGenotype<f32>>,
        impl StrategyReporter<Genotype = MultiRangeGenotype<f32>>,
    >,
    name: &str,
    run: usize,
    duration: std::time::Duration,
) {
    if let Some((best_genes, fitness_score)) = evolve.best_genes_and_fitness_score() {
        // Genes are already f32 for MultiRangeGenotype<f32>
        let x1 = best_genes[0];
        let x2 = best_genes[1].floor();
        let result = 8.0 * x1 + x2;

        println!(
            "[{}] Run {}: x1: {:.5}, x2: {} => {:.5} (fitness: {}, generation: {}, duration: {:?})",
            name,
            run,
            x1,
            x2 as u8,
            result,
            fitness_score,
            evolve.best_generation(),
            duration
        );
    } else {
        println!(
            "[{}] Run {}: No valid solution found (duration: {:?})",
            name, run, duration
        );
    }
}
