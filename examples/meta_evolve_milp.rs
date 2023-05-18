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
    let rounds = 10;
    let population_sizes = vec![
        //10, //20,
        //50,
        //100, //200,
        //500,
        1000,
    ];
    let max_stale_generations_options = vec![Some(1000)];
    let target_fitness_score_options = vec![Some(0)];
    let mass_degeneration_options = vec![
        None,
        //Some(MassDegeneration::new(0.9, 10)),
        //Some(MassDegeneration::new(0.99, 100)),
        //Some(MassDegeneration::new(0.99, 10)),
    ];
    let mass_extinction_options = vec![
        None,
        //Some(MassExtinction::new(0.9, 0.1)),
        //Some(MassExtinction::new(0.99, 0.01)),
        //Some(MassExtinction::new(0.99, 0.1)),
    ];
    let mass_genesis_options = vec![
        None,
        //Some(MassGenesis::new(0.9)),
        //Some(MassGenesis::new(0.99)),
    ];
    let mass_invasion_options = vec![
        None,
        Some(MassInvasion::new(0.9, 0.1)),
        Some(MassInvasion::new(0.99, 0.01)),
        Some(MassInvasion::new(0.99, 0.1)),
    ];
    let mutates = vec![
        //MutateDispatch(Mutates::Once, 0.05),
        MutateDispatch(Mutates::Once, 0.1),
        MutateDispatch(Mutates::Once, 0.2),
        MutateDispatch(Mutates::Once, 0.3),
        //MutateDispatch(Mutates::Once, 0.4),
        //MutateDispatch(Mutates::Once, 0.5),
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
        //CompeteDispatch(Competes::Elite, 0),
        //CompeteDispatch(Competes::Tournament, 2),
        CompeteDispatch(Competes::Tournament, 4),
        //CompeteDispatch(Competes::Tournament, 8),
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
        .with_population_sizes(population_sizes)
        .with_max_stale_generations_options(max_stale_generations_options)
        .with_target_fitness_score_options(target_fitness_score_options)
        .with_mass_degeneration_options(mass_degeneration_options)
        .with_mass_extinction_options(mass_extinction_options)
        .with_mass_genesis_options(mass_genesis_options)
        .with_mass_invasion_options(mass_invasion_options)
        .with_mutates(mutates)
        .with_crossovers(crossovers)
        .with_competes(competes)
        .build()
        .unwrap();

    let permutate = MetaPermutate::new(&config).call();
    println!();
    println!("{}", permutate);
}
