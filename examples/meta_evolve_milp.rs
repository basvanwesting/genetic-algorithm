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
        //MutateOnce::new_dispatch(0.05),
        //MutateOnce::new_dispatch(0.1),
        MutateOnce::new_dispatch(0.2),
        //MutateOnce::new_dispatch(0.3),
        //MutateOnce::new_dispatch(0.4),
        //MutateOnce::new_dispatch(0.5),
    ];
    let crossovers = vec![
        //CrossoverDispatch(Crossovers::Clone, false),
        //CrossoverDispatch(Crossovers::Clone, true),
        //CrossoverDispatch(Crossovers::SingleGene, false),
        //CrossoverDispatch(Crossovers::SingleGene, true),
        //CrossoverDispatch(Crossovers::SinglePoint, false),
        //CrossoverDispatch(Crossovers::SinglePoint, true),
        //CrossoverDispatch(Crossovers::Uniform, false),
        CrossoverDispatch(Crossovers::Uniform, true),
    ];
    let competes = vec![
        CompeteElite::new_dispatch(),
        CompeteTournament::new_dispatch(2),
        CompeteTournament::new_dispatch(4),
        CompeteTournament::new_dispatch(8),
    ];
    let extensions = vec![
        ExtensionNoop::new_dispatch(),
        ExtensionMassDegeneration::new_dispatch(0.99, 100),
        ExtensionMassExtinction::new_dispatch(0.99, 0.01),
        ExtensionMassGenesis::new_dispatch(0.99),
        ExtensionMassInvasion::new_dispatch(0.99, 0.01),
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
