use genetic_algorithm::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 100;
const POPULATION_SIZE: usize = 100;

#[derive(Clone, Debug)]
pub struct DistanceTo(pub i32); // target
impl Fitness for DistanceTo {
    type Genotype = RangeGenotype<i32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .map(|v| (v - self.0).abs() as FitnessValue)
                .sum(),
        )
    }
}

fn main() {
    env_logger::init();

    let genotype = RangeGenotype::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0..=10)
        .with_mutation_type(MutationType::Range(1))
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(POPULATION_SIZE)
        .with_max_stale_generations(100)
        .with_target_fitness_score(1)
        .with_fitness(DistanceTo(5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        // .with_par_fitness(true) // 2x slower in this case
        .with_mutate(MutateMultiGene::new(2, 0.2))
        .with_crossover(CrossoverMultiPoint::new(0.7, 0.8, 9, false))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_reporter(EvolveReporterSimple::new_with_buffer(100))
        // .with_reporter(EvolveReporterSimple::new(100))
        .call()
        .unwrap();

    println!("{}", evolve);

    let mut buffer = vec![];
    evolve.flush_reporter(&mut buffer);
    print!("{}", String::from_utf8(buffer).unwrap());
}
