use genetic_algorithm::centralized::crossover::CrossoverUniform;
use genetic_algorithm::centralized::fitness::{Fitness, FitnessOrdering};
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::mutate::MutateMultiGene;
use genetic_algorithm::centralized::select::SelectElite;
use genetic_algorithm::centralized::strategy::evolve::{Evolve, EvolveReporterSimple};
use genetic_algorithm::centralized::strategy::Strategy;

const GENES_SIZE: usize = 100;
const POPULATION_SIZE: usize = 200;

// Count the number of true values in the chromosome
#[derive(Clone, Debug)]
pub struct CountOnes;
impl Fitness for CountOnes {
    type Genotype = StaticBinaryGenotype<GENES_SIZE, POPULATION_SIZE>;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &<Self::Genotype as Genotype>::Chromosome,
        genotype: &Self::Genotype,
    ) -> Option<isize> {
        Some(
            genotype
                .genes_slice(chromosome)
                .iter()
                .filter(|&&x| x)
                .count() as isize,
        )
    }
}

fn main() {
    env_logger::init();

    let genotype = StaticBinaryGenotype::<GENES_SIZE, POPULATION_SIZE>::builder()
        .with_genes_size(GENES_SIZE)
        .build()
        .unwrap();

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_select(SelectElite::new(0.1, 0.9))
        .with_crossover(CrossoverUniform::new(0.5, 0.5))
        .with_mutate(MutateMultiGene::new(3, 0.01))
        .with_fitness(CountOnes)
        .with_fitness_ordering(FitnessOrdering::Maximize)
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_reporter(EvolveReporterSimple::new(10))
        .build()
        .unwrap();

    evolve.call();

    let (best_genes, best_fitness) = evolve.best_genes_and_fitness_score().unwrap();

    println!("\nBest solution found:");
    println!("Number of true values: {}", best_fitness);
    println!("First 20 genes: {:?}", &best_genes[..20]);
}
