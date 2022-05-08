use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::{CompeteDispatch, Competes};
use genetic_algorithm::crossover::{CrossoverDispatch, Crossovers};
use genetic_algorithm::evolve::EvolveBuilder;
use genetic_algorithm::fitness::{Fitness, FitnessOrdering, FitnessValue};
use genetic_algorithm::genotype::{Genotype, UniqueIndexGenotype};
use genetic_algorithm::meta::{MetaConfig, MetaPermutate};
use genetic_algorithm::mutate::{MutateDispatch, Mutates};

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueIndexGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let mut score = 0;
        let max_index = chromosome.genes.len() - 1;
        for i in 0..max_index {
            for j in 0..max_index {
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

fn main() {
    let rounds = 100;
    let population_sizes = vec![20];
    let max_stale_generations_options = vec![Some(10000)];
    let target_fitness_score_options = vec![Some(0)];
    let degeneration_range_options = vec![None, Some(0.001..0.999)];
    let mutates = vec![
        MutateDispatch(Mutates::Once, 0.1),
        MutateDispatch(Mutates::Once, 0.2),
        MutateDispatch(Mutates::Once, 0.3),
    ];
    let crossovers = vec![CrossoverDispatch(Crossovers::Clone, true)];
    let competes = vec![CompeteDispatch(Competes::Elite, 0)];
    let genotype = UniqueIndexGenotype::builder()
        .with_gene_value_size(64)
        .build()
        .unwrap();
    let fitness = NQueensFitness;

    let evolve_builder = EvolveBuilder::new()
        .with_genotype(genotype)
        .with_fitness(fitness)
        .with_fitness_ordering(FitnessOrdering::Minimize);
    let evolve_fitness_to_micro_second_factor = 1_000_000;

    let config = MetaConfig::builder()
        .with_evolve_builder(evolve_builder)
        .with_evolve_fitness_to_micro_second_factor(evolve_fitness_to_micro_second_factor)
        .with_rounds(rounds)
        .with_population_sizes(population_sizes)
        .with_max_stale_generations_options(max_stale_generations_options)
        .with_target_fitness_score_options(target_fitness_score_options)
        .with_degeneration_range_options(degeneration_range_options)
        .with_mutates(mutates)
        .with_crossovers(crossovers)
        .with_competes(competes)
        .build()
        .unwrap();

    let permutate = MetaPermutate::new(&config).call();

    println!();
    println!("{}", permutate);
}
