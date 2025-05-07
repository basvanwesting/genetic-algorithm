use genetic_algorithm::fitness::placeholders::CountTrue;
use genetic_algorithm::strategy::prelude::*;

fn main() {
    env_logger::init();

    // small enought for permutate
    let genotype = BinaryGenotype::builder()
        .with_genes_size(16)
        .build()
        .unwrap();

    let builder = StrategyBuilder::new()
        .with_genotype(genotype)
        .with_target_population_size(20)
        .with_max_stale_generations(20)
        // .with_target_fitness_score(16) // short-circuits
        .with_max_stale_generations(10)
        .with_fitness(CountTrue)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_crossover(CrossoverClone::new(0.5))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_reporter(IterationReporter);

    let strategies = [
        StrategyVariant::Permutate(PermutateVariant::Standard),
        StrategyVariant::Evolve(EvolveVariant::Standard),
        StrategyVariant::HillClimb(HillClimbVariant::Stochastic),
        StrategyVariant::HillClimb(HillClimbVariant::SteepestAscent),
    ];

    strategies.iter().copied().for_each(|variant| {
        println!("call: {}", variant);
        let strategy = builder.clone().with_variant(variant).call().unwrap();

        if let Some(fitness_score) = strategy.best_fitness_score() {
            println!("  fitness score: {}", fitness_score);
        } else {
            println!("  invalid solution with fitness score: None");
        }
    });

    strategies.iter().copied().for_each(|variant| {
        println!("call_repeatedly(3): {}", variant);
        let (mut strategy, mut others) = builder
            .clone()
            .with_variant(variant)
            .call_repeatedly(3)
            .unwrap();

        let other_fitness_scores = others
            .iter()
            .map(|s| s.best_fitness_score())
            .collect::<Vec<_>>();

        let mut buffer = Vec::<u8>::new();
        strategy.flush_reporter(&mut buffer);
        others
            .iter_mut()
            .for_each(|s| s.flush_reporter(&mut buffer));
        println!(
            "  all reporter buffers: {}",
            String::from_utf8(buffer).unwrap_or_default()
        );

        if let Some(fitness_score) = strategy.best_fitness_score() {
            println!(
                "  best fitness score: {}, others: {:?}",
                fitness_score, other_fitness_scores
            );
        } else {
            println!("  invalid solution with fitness score: None");
        }
    });

    strategies.iter().copied().for_each(|variant| {
        println!("call_par_speciated(3): {}", variant);
        let (mut strategy, mut others) = builder
            .clone()
            .with_variant(variant)
            .call_par_speciated(3)
            .unwrap();

        let other_fitness_scores = others
            .iter()
            .map(|s| s.best_fitness_score())
            .collect::<Vec<_>>();

        let mut buffer = Vec::<u8>::new();
        strategy.flush_reporter(&mut buffer);
        others
            .iter_mut()
            .for_each(|s| s.flush_reporter(&mut buffer));
        println!(
            "  all reporter buffers: {}",
            String::from_utf8(buffer).unwrap_or_default()
        );

        if let Some(fitness_score) = strategy.best_fitness_score() {
            println!(
                "  best fitness score: {}, others: {:?}",
                fitness_score, other_fitness_scores
            );
        } else {
            println!("  invalid solution with fitness score: None");
        }
    });
}

#[derive(Clone)]
pub struct IterationReporter;
impl StrategyReporter for IterationReporter {
    type Genotype = BinaryGenotype;

    fn flush(&mut self, output: &mut Vec<u8>) {
        output.push(b'.')
    }
    fn on_start<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        let number_of_seed_genes = genotype.seed_genes_list().len();
        if number_of_seed_genes > 0 {
            println!(
                "  start - iteration: {}, number of seed genes: {:?}",
                state.current_iteration(),
                number_of_seed_genes
            );
        } else {
            println!("  start - iteration: {}", state.current_iteration());
        }
    }
    fn on_finish<S: StrategyState<Self::Genotype>, C: StrategyConfig>(
        &mut self,
        _genotype: &Self::Genotype,
        state: &S,
        _config: &C,
    ) {
        println!("  finish - iteration: {}", state.current_iteration());
    }
}
