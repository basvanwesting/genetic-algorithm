use genetic_algorithm::centralized::genotype::StaticRangeGenotype;
use genetic_algorithm::centralized::strategy::evolve::prelude::*;

const GENES_SIZE: usize = 100;
const POPULATION_SIZE: usize = 100;
const SELECTION_RATE: f32 = 0.7;
const MATRIX_POP_SIZE: usize =
    POPULATION_SIZE + (POPULATION_SIZE as f32 * SELECTION_RATE + 1.0) as usize;

#[derive(Clone, Debug)]
pub struct DistanceTo(pub i32); // target
impl Fitness for DistanceTo {
    type Genotype = StaticRangeGenotype<i32, GENES_SIZE, MATRIX_POP_SIZE>;
    fn calculate_for_population(
        &mut self,
        _population: &Population,
        genotype: &FitnessGenotype<Self>,
    ) -> Vec<Option<FitnessValue>> {
        // pure matrix data calculation on [[T; N] M]
        // the order of the rows needs to be preserved as it matches the row_id on the chromosome
        genotype
            .data
            .iter()
            .map(|genes| {
                genes
                    .iter()
                    .map(|v| (v - self.0).abs() as isize)
                    .sum::<isize>() as FitnessValue
            })
            .map(Some)
            .collect()
    }
}

fn main() {
    env_logger::init();

    let genotype = StaticRangeGenotype::<i32, GENES_SIZE, MATRIX_POP_SIZE>::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0..=10)
        .with_allele_mutation_range(-1..=1)
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
        .with_mutate(MutateMultiGene::new(2, 0.2))
        .with_crossover(CrossoverMultiPoint::new(SELECTION_RATE, 0.8, 9, false))
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
