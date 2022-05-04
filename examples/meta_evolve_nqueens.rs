use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::compete::{CompeteDispatch, Competes};
use genetic_algorithm::crossover::{CrossoverDispatch, Crossovers};
use genetic_algorithm::fitness::Fitness;
use genetic_algorithm::genotype::UniqueIndexGenotype;
use genetic_algorithm::meta::{MetaConfig, MetaPermutate};
use genetic_algorithm::mutate::{MutateDispatch, Mutates};

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueIndexGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        let mut score = 0;
        let max_index = chromosome.genes.len() - 1;
        for i in 0..max_index {
            for j in 0..max_index {
                if i != j {
                    let dx = i.abs_diff(j);
                    let dy = chromosome.genes[i].abs_diff(chromosome.genes[j]) as usize;
                    if dx == dy {
                        //diagonal clash
                        score -= 1;
                    }
                }
            }
        }
        score
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
    let evolve_genotype = UniqueIndexGenotype::new().with_gene_value_size(64).build();
    let evolve_fitness = NQueensFitness;

    let config = MetaConfig::new(
        rounds,
        evolve_genotype,
        evolve_fitness,
        population_sizes,
        max_stale_generations_options,
        target_fitness_score_options,
        degeneration_range_options,
        mutates,
        crossovers,
        competes,
    );

    let permutate = MetaPermutate::new(&config).call();

    println!();
    println!("{}", permutate);
}
