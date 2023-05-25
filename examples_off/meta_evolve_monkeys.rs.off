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
    env_logger::init();

    let rounds = 10;
    let target_population_sizes = vec![20, 50, 100];
    let max_stale_generations_options = vec![Some(10000)];
    let target_fitness_score_options = vec![Some(0)];
    let mutates = vec![
        MutateOnce::new(0.1).into(),
        MutateOnce::new(0.2).into(),
        MutateOnce::new(0.3).into(),
    ];
    let crossovers = vec![
        CrossoverSinglePoint::new(true).into(),
        CrossoverSinglePoint::new(false).into(),
        CrossoverSingleGene::new(false).into(),
        CrossoverClone::new(true).into(),
    ];
    let competes = vec![CompeteElite::new().into(), CompeteTournament::new(4).into()];
    let extensions = vec![
        ExtensionNoop::new().into(),
        //ExtensionMassDegeneration::new(0.9, 10).into(),
        //ExtensionMassExtinction::new(0.9, 0.1).into(),
        //ExtensionMassGenesis::new(0.9).into(),
        //ExtensionMassInvasion::new(0.9, 0.1).into(),
    ];
    let genotype = DiscreteGenotype::builder()
        .with_genes_size(TARGET_TEXT.len())
        .with_allele_list((MIN_CHAR..=MAX_CHAR).collect())
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
        .with_target_population_sizes(target_population_sizes)
        .with_max_stale_generations_options(max_stale_generations_options)
        .with_target_fitness_score_options(target_fitness_score_options)
        .with_mutates(mutates)
        .with_crossovers(crossovers)
        .with_competes(competes)
        .with_extensions(extensions)
        .build()
        .unwrap();

    let permutate = MetaPermutate::new(&config).call();

    println!();
    println!("{}", permutate);
}
