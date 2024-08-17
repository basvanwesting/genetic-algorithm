use genetic_algorithm::strategy::hill_climb::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Clone, Debug)]
pub struct DistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DistanceTo {
    type Allele = f32;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .map(|v| (v - self.0).abs() / self.1)
                .sum::<Self::Allele>() as FitnessValue,
        )
    }
}

fn main() {
    env_logger::init();

    let mut rng = SmallRng::from_entropy();
    let genotype = ContinuousGenotype::builder()
        .with_genes_size(100)
        .with_allele_range(0.0..=1.0)
        // .with_allele_neighbour_range(-0.1..=0.1) // won't converge
        // .with_allele_neighbour_range(-0.001..=0.001) // slow converge
        .with_allele_neighbour_scaled_range(vec![
            -0.1..=0.1,
            -0.05..=0.05,
            -0.025..=0.025,
            -0.01..=0.01,
            -0.005..=0.005,
            -0.0025..=0.0025,
            -0.001..=0.001,
            -0.0005..=0.0005,
            -0.00025..=0.00025,
            -0.0001..=0.0001,
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let now = std::time::Instant::now();

    let hill_climb = HillClimb::builder()
        .with_genotype(genotype)
        // .with_variant(HillClimbVariant::Stochastic)
        // .with_max_stale_generations(10000)
        .with_variant(HillClimbVariant::SteepestAscent)
        .with_max_stale_generations(1)
        .with_target_fitness_score(100 * 100)
        .with_fitness(DistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_reporter(HillClimbReporterSimple::new_with_flags(1000, false, true))
        .call(&mut rng)
        .unwrap();

    let duration = now.elapsed();

    println!("{}", hill_climb);
    println!("duration: {:?}", duration);
}
