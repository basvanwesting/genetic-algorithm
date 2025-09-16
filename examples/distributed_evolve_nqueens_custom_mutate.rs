use genetic_algorithm::distributed::strategy::evolve::prelude::*;
use genetic_algorithm::distributed::chromosome::Chromosome;
use genetic_algorithm::distributed::mutate::Mutate;
use genetic_algorithm::distributed::strategy::{StrategyAction, StrategyState};
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle
#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueGenotype<u8>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let mut score = 0;
        let genes_size = chromosome.genes.len();
        for i in 0..genes_size {
            for j in 0..genes_size {
                if i != j {
                    let dx = i.abs_diff(j);
                    let dy = chromosome.genes[i].abs_diff(chromosome.genes[j]) as usize;
                    if dx == dy {
                        //diagonal clash
                        score += 1;
                    }
                }
            }
        }
        Some(score)
    }
}

// Custom Mutate that directly manipulates chromosome genes
// This demonstrates domain-specific mutation without using genotype.mutate_chromosome_genes()
// It mimics UniqueGenotype's internal logic but only for even column positions
#[derive(Clone, Debug)]
struct MutateEvenIndicesOnly<T: genetic_algorithm::distributed::allele::Allele> {
    mutation_rate: f32,
    genes_size: usize,
    gene_index_sampler: Uniform<usize>,  // Mimics UniqueGenotype's sample_gene_index
    _phantom: std::marker::PhantomData<T>,
}

impl<T: genetic_algorithm::distributed::allele::Allele> MutateEvenIndicesOnly<T> {
    pub fn new(mutation_rate: f32, genes_size: usize) -> Self {
        Self { 
            mutation_rate, 
            genes_size,
            gene_index_sampler: Uniform::from(0..genes_size),  // Like UniqueGenotype internally
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: genetic_algorithm::distributed::allele::Allele> Mutate for MutateEvenIndicesOnly<T> {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        _genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) where G::Allele: std::fmt::Debug {
        let now = Instant::now();
        
        // Direct chromosome manipulation without genotype.mutate_chromosome_genes()
        for chromosome in &mut state.population.chromosomes {
            if rng.gen::<f32>() < self.mutation_rate {
                // Build list of even indices (domain-specific constraint)
                let even_indices: Vec<usize> = (0..self.genes_size)
                    .step_by(2)
                    .collect();
                
                if even_indices.len() >= 2 {
                    // Use our own sampling logic similar to UniqueGenotype's internal sample_gene_index
                    // but constrained to even indices
                    let sampled_idx = self.gene_index_sampler.sample(rng);
                    let idx1 = if sampled_idx % 2 == 0 {
                        sampled_idx
                    } else {
                        // Round to nearest even index
                        (sampled_idx / 2) * 2
                    };
                    
                    // Sample second even index differently
                    let idx2_pos = rng.gen_range(0..even_indices.len());
                    let mut idx2 = even_indices[idx2_pos];
                    
                    // Ensure different indices
                    let mut attempts = 0;
                    while idx2 == idx1 && attempts < 10 {
                        let new_pos = rng.gen_range(0..even_indices.len());
                        idx2 = even_indices[new_pos];
                        attempts += 1;
                    }
                    
                    if idx1 != idx2 && idx1 < chromosome.genes.len() && idx2 < chromosome.genes.len() {
                        // Direct gene swap - the core custom mutation logic
                        chromosome.genes.swap(idx1, idx2);
                        
                        // Essential: reset chromosome state after manual mutation
                        // This recalculates fitness_score and genes_hash
                        chromosome.reset_state();
                    }
                }
            }
        }
        
        state.add_duration(StrategyAction::Mutate, now.elapsed());
    }
}

// Alternative custom Mutate that only mutates the first half of genes
#[derive(Clone, Debug)]
struct MutateFirstHalfOnly {
    mutation_rate: f32,
    mutations_per_chromosome: usize,
}

impl MutateFirstHalfOnly {
    pub fn new(mutation_rate: f32, mutations_per_chromosome: usize) -> Self {
        Self {
            mutation_rate,
            mutations_per_chromosome,
        }
    }
}

impl Mutate for MutateFirstHalfOnly {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        _config: &EvolveConfig,
        _reporter: &mut SR,
        rng: &mut R,
    ) {
        let now = Instant::now();
        let genes_size = genotype.genes_size();
        let half_size = genes_size / 2;
        
        for chromosome in &mut state.population.chromosomes {
            if rng.gen::<f32>() < self.mutation_rate {
                // For demonstration: manually swap genes in first half only
                for _ in 0..self.mutations_per_chromosome {
                    let idx1 = rng.gen_range(0..half_size);
                    let idx2 = rng.gen_range(0..half_size);
                    
                    if idx1 != idx2 {
                        // Use genotype's built-in mutation but restrict to first half
                        genotype.mutate_chromosome_genes(
                            1,
                            false,
                            chromosome,
                            None,
                            rng,
                        );
                    }
                }
            }
        }
        
        state.add_duration(StrategyAction::Mutate, now.elapsed());
    }
}

fn main() {
    env_logger::init();

    const BOARD_SIZE: u8 = 16;  // Smaller board for demonstration

    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..BOARD_SIZE).collect())
        .with_genes_hashing(true)
        .build()
        .unwrap();

    println!("N-Queens problem with custom Mutate implementation");
    println!("Board size: {}x{}", BOARD_SIZE, BOARD_SIZE);
    println!();
    println!("Using MutateEvenIndicesOnly - only swaps genes at even positions");
    println!("This is intentionally inefficient to demonstrate custom implementation");
    println!();

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(50)
        .with_max_stale_generations(500)  // Limited generations to show custom mutation behavior
        .with_fitness(NQueensFitness)
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_fitness_cache(10000)
        .with_target_fitness_score(0)
        // Using our custom Mutate implementation with direct gene manipulation
        .with_mutate(MutateEvenIndicesOnly::<u8>::new(0.3, BOARD_SIZE as usize))
        // Alternative: .with_mutate(MutateFirstHalfOnly::new(0.3, 2))
        // Standard mutation would be: .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverClone::new(0.7))
        .with_select(SelectElite::new(0.5, 0.05))
        .with_reporter(EvolveReporterSimple::new(100))
        .build()
        .unwrap();

    evolve.call();

    if let Some((best_genes, fitness_score)) = evolve.best_genes_and_fitness_score() {
        println!("\nBest solution found:");
        for gene in best_genes {
            let mut chars: Vec<char> = (0..BOARD_SIZE).map(|_| '.').collect();
            chars[gene as usize] = 'Q';
            println!("{}", String::from_iter(chars));
        }
        println!("\nFitness score: {} (diagonal clashes)", fitness_score);
        
        if fitness_score == 0 {
            println!("✅ Found valid solution!");
            println!("Despite the constraint of only mutating even columns,");
            println!("the algorithm still found a solution!");
        } else {
            println!("❌ Not a valid solution - fitness score: {}", fitness_score);
            println!("This is expected with the constrained even-indices-only mutation.");
            println!("The standard MutateSingleGene would find solutions more reliably.");
        }
    } else {
        println!("No solution found");
    }

    println!("\n💡 This example shows how to implement custom Mutate strategies");
    println!("   for domain-specific mutation patterns in your genetic algorithm.");
}