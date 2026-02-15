//! Custom Extension implementation for the Evolve strategy.
//! Demonstrates how to implement the Extension trait for custom diversity management.
use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::strategy::evolve::prelude::*;
use rand::Rng;

/// Custom extension that demonstrates using multiple extension points.
/// All extension hooks are optional as shown in default after_mutation_complete noop
#[derive(Clone, Debug)]
pub struct MultiPointExtension {
    pub selection_threshold: usize,
}

impl Extension for MultiPointExtension {
    type Genotype = BinaryGenotype;

    // After selection: Mass extinction if diversity is too low
    fn after_selection_complete<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &mut Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        config: &EvolveConfig,
        reporter: &mut SR,
        _rng: &mut R,
    ) {
        if let Some(cardinality) = state.population_cardinality() {
            if cardinality <= self.selection_threshold {
                println!(
                    "After selection: Low diversity detected ({}), applying mass extinction",
                    cardinality
                );

                reporter.on_extension_event(
                    ExtensionEvent("MassExtinctionAfterSelection".to_string()),
                    genotype,
                    state,
                    config,
                );

                // Keep only best 20% of population
                let keep_size = (state.population.size() as f32 * 0.2).ceil() as usize;
                let mut elite = self.extract_elite_chromosomes(genotype, state, config, keep_size);
                state.population.truncate(2);
                state.population.chromosomes.append(&mut elite);
            }
        }
    }

    // After crossover: Log statistics
    fn after_crossover_complete<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        _genotype: &mut Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        _rng: &mut R,
    ) {
        let avg_age = state
            .population
            .chromosomes
            .iter()
            .map(|c| c.age())
            .sum::<usize>() as f64
            / state.population.size() as f64;
        println!(
            "After crossover: Population size: {}, Avg age: {:.2}",
            state.population.size(),
            avg_age
        );
    }

    // After mutation: Default Noop

    // After generation: Remove duplicates if too many
    fn after_generation_complete<R: Rng, SR: StrategyReporter<Genotype = Self::Genotype>>(
        &mut self,
        genotype: &mut Self::Genotype,
        state: &mut EvolveState<Self::Genotype>,
        config: &EvolveConfig,
        reporter: &mut SR,
        _rng: &mut R,
    ) {
        if genotype.genes_hashing() {
            let unique_count = state.population.unique_chromosome_indices().len();
            let total_count = state.population.size();
            let duplicate_ratio = 1.0 - (unique_count as f64 / total_count as f64);

            if duplicate_ratio > 0.5 {
                println!(
                    "After generation: High duplication ratio ({:.2}%), removing duplicates",
                    duplicate_ratio * 100.0
                );

                reporter.on_extension_event(
                    ExtensionEvent("RemoveDuplicates".to_string()),
                    genotype,
                    state,
                    config,
                );

                let mut unique = self.extract_unique_chromosomes(genotype, state, config);
                let remaining = 2usize.saturating_sub(unique.len());
                state.population.truncate(remaining);
                state.population.chromosomes.append(&mut unique);
            }
        }
    }
}

fn main() {
    println!("Starting evolution with multiple extension points...\n");

    let genotype = BinaryGenotype::builder()
        .with_genes_size(50)
        .with_genes_hashing(true) // Required for deduplication
        .with_chromosome_recycling(true)
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_select(SelectElite::new(0.9, 0.02))
        .with_crossover(CrossoverUniform::new(0.8, 0.8))
        .with_mutate(MutateSingleGene::new(0.1))
        .with_fitness(CountTrue)
        .with_fitness_ordering(FitnessOrdering::Maximize)
        .with_extension(MultiPointExtension {
            selection_threshold: 10,
        })
        .with_target_population_size(50)
        .with_target_fitness_score(50) // All true
        .with_max_stale_generations(200)
        .with_max_generations(10_000)
        .with_rng_seed_from_u64(42) // Deterministic for demo
        .call()
        .unwrap();

    let (best_genes, best_fitness) = evolve.best_genes_and_fitness_score().unwrap();
    let true_count = best_genes.iter().filter(|&&g| g).count();

    println!("\n=== Evolution complete ===");
    println!("Best fitness: {}", best_fitness);
    println!("True genes: {}/{}", true_count, best_genes.len());
    println!("Total generations: {}", evolve.state.current_generation);
    println!("Best found at generation: {}", evolve.state.best_generation);
}
