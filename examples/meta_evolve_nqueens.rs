use genetic_algorithm::meta::prelude::*;

// see https://en.wikipedia.org/wiki/Eight_queens_puzzle

#[derive(Clone, Debug)]
struct NQueensFitness;
impl Fitness for NQueensFitness {
    type Genotype = UniqueGenotype<u8>;
    fn calculate_for_chromosome(
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
    env_logger::init();

    let rounds = 100;
    let population_sizes = vec![20];
    let max_stale_generations_options = vec![Some(10000)];
    let target_fitness_score_options = vec![Some(0)];
    let mutates = vec![
        MutateOnce::new_dispatch(0.1),
        MutateOnce::new_dispatch(0.2),
        MutateOnce::new_dispatch(0.3),
    ];
    let crossovers = vec![CrossoverDispatch(Crossovers::Clone, true)];
    let competes = vec![CompeteDispatch(Competes::Elite, 0)];
    let extensions = vec![
        ExtensionNoop::new_dispatch(),
        //ExtensionMassDegeneration::new_dispatch(0.9, 10),
        //ExtensionMassExtinction::new_dispatch(0.9, 0.1),
        //ExtensionMassGenesis::new_dispatch(0.9),
        //ExtensionMassInvasion::new_dispatch(0.9, 0.1),
    ];
    let genotype = UniqueGenotype::builder()
        .with_allele_list((0..64).collect())
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
