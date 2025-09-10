use genetic_algorithm::centralized::genotype::StaticRangeGenotype;
use genetic_algorithm::centralized::strategy::hill_climb::prelude::*;

const GENES_SIZE: usize = 100;
const MAX_POPULATION_SIZE: usize = 300; // For static genotype capacity

#[derive(Clone, Debug)]
pub struct DistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DistanceTo {
    type Genotype = StaticRangeGenotype<f32, GENES_SIZE, MAX_POPULATION_SIZE>;
    fn calculate_for_population(
        &mut self,
        _population: &Population,
        genotype: &FitnessGenotype<Self>,
    ) -> Vec<Option<FitnessValue>> {
        genotype
            .data
            .iter()
            .map(|genes| {
                genes
                    .iter()
                    .map(|v| (v - self.0).abs() / self.1)
                    .sum::<f32>() as FitnessValue
            })
            .map(Some)
            .collect()
    }
}

fn main() {
    env_logger::init();

    let genotype = StaticRangeGenotype::<f32, GENES_SIZE, MAX_POPULATION_SIZE>::builder()
        .with_genes_size(GENES_SIZE)
        .with_allele_range(0.0..=1.0)
        // .with_allele_mutation_range(-0.1..=0.1) // won't converge for SteepestAscent
        // .with_allele_mutation_range(-0.001..=0.001) // slow converge
        .with_allele_mutation_scaled_range(vec![
            -0.1..=0.1,
            -0.01..=0.01,
            -0.001..=0.001,
            -0.0001..=0.0001,
            -0.00001..=0.00001,
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        // .with_variant(HillClimbVariant::Stochastic)
        // .with_max_stale_generations(10000)
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_max_stale_generations(2)
        .with_target_fitness_score((GENES_SIZE as f32 * 0.001 / 1e-5) as isize)
        .with_fitness(DistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_reporter(HillClimbReporterSimple::new(100))
        .call()
        .unwrap();

    println!("{}", hill_climb);
}
