use genetic_algorithm::meta::prelude::*;

const LP: f32 = 59.0;
const PRECISION: f32 = 1e-5;

// see https://www.mathworks.com/help/optim/ug/intlinprog.html#bts3gkc-2
#[derive(Clone, Debug)]
struct MILPFitness;
impl Fitness for MILPFitness {
    type Genotype = MultiContinuousGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let x1 = chromosome.genes[0];
        let x2 = chromosome.genes[1].floor();

        if x1 + 2.0 * x2 >= 14.0 && -4.0 * x1 - x2 <= 33.0 && 2.0 * x1 + x2 <= 20.0 {
            let score = (8.0 * x1 + x2 - LP).abs();
            Some((score / PRECISION) as isize)
        } else {
            None
        }
    }
}

fn main() {
    env_logger::init();

    let rounds = 10;
    let target_population_sizes = vec![
        //10, //20,
        //50,
        //100, //200,
        //500,
        1000,
    ];
    let max_stale_generations_options = vec![Some(1000)];
    let target_fitness_score_options = vec![Some(0)];
    let mutates = vec![
        //MutateOnce::new(0.05).into(),
        //MutateOnce::new(0.1).into(),
        MutateOnce::new(0.2).into(),
        //MutateOnce::new(0.3).into(),
        //MutateOnce::new(0.4).into(),
        //MutateOnce::new(0.5).into(),
    ];
    let crossovers = vec![
        //CrossoverClone::new(false).into(),
        //CrossoverClone::new(true).into(),
        //CrossoverSingleGene::new(false).into(),
        //CrossoverSingleGene::new(true).into(),
        //CrossoverSinglePoint::new(false).into(),
        //CrossoverSinglePoint::new(true).into(),
        //CrossoverUniform::new(false).into(),
        CrossoverUniform::new(true).into(),
    ];
    let competes = vec![
        CompeteElite::new().into(),
        CompeteTournament::new(2).into(),
        CompeteTournament::new(4).into(),
        CompeteTournament::new(8).into(),
    ];
    let extensions = vec![
        ExtensionNoop::new().into(),
        ExtensionMassDegeneration::new(0.99, 100).into(),
        ExtensionMassExtinction::new(0.99, 0.01).into(),
        ExtensionMassGenesis::new(0.99).into(),
        ExtensionMassInvasion::new(0.99, 0.01).into(),
    ];
    let genotype = MultiContinuousGenotype::builder()
        .with_allele_ranges(vec![(-10.0..10.0), (0.0..10.0)])
        .build()
        .unwrap();
    let fitness = MILPFitness;
    let evolve_builder = EvolveBuilder::new()
        .with_genotype(genotype)
        .with_fitness(fitness)
        .with_fitness_ordering(FitnessOrdering::Minimize);
    let evolve_fitness_to_micro_second_factor = 10_000_000;

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
