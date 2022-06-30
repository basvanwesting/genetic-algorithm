use distance::hamming;
use genetic_algorithm::meta::prelude::*;

// see https://en.wikipedia.org/wiki/Infinite_monkey_theorem

const TARGET_TEXT: &str =
    "Some are great, some achieve greatness, and some have greatness thrust upon 'em.";

// printable chars
const MIN_CHAR: char = ' '; // 0x20;
const MAX_CHAR: char = '~'; // 0x7e;

#[derive(Clone, Debug)]
struct MonkeyFitness;
impl Fitness for MonkeyFitness {
    type Genotype = DiscreteGenotype<char>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let string = String::from_iter(chromosome.genes.clone());
        Some(hamming(&string, TARGET_TEXT).unwrap() as FitnessValue)
    }
}

fn main() {
    let rounds = 10;
    let population_sizes = vec![20, 50, 100];
    let max_stale_generations_options = vec![Some(10000)];
    let target_fitness_score_options = vec![Some(0)];
    let degeneration_range_options = vec![None, Some(0.001..0.999)];
    let mutates = vec![
        MutateDispatch(Mutates::Once, 0.1),
        MutateDispatch(Mutates::Once, 0.2),
        MutateDispatch(Mutates::Once, 0.3),
    ];
    let crossovers = vec![
        CrossoverDispatch(Crossovers::SinglePoint, true),
        CrossoverDispatch(Crossovers::SinglePoint, false),
        CrossoverDispatch(Crossovers::SingleGene, false),
        CrossoverDispatch(Crossovers::Clone, true),
    ];
    let competes = vec![
        CompeteDispatch(Competes::Elite, 0),
        CompeteDispatch(Competes::Tournament, 4),
    ];
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(TARGET_TEXT.len())
        .with_allele_values((MIN_CHAR..=MAX_CHAR).collect())
        .build()
        .unwrap();
    let fitness = MonkeyFitness;

    let evolve_builder = EvolveBuilder::new()
        .with_genotype(genotype)
        .with_fitness(fitness)
        .with_fitness_ordering(FitnessOrdering::Minimize);
    let evolve_fitness_to_micro_second_factor = 1_000_000_000;

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
