use genetic_algorithm::strategy::evolve::prelude::*;
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
        .with_allele_neighbour_range(-0.1..=0.1)
        .build()
        .unwrap();

    println!("{}", genotype);

    let now = std::time::Instant::now();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(100)
        .with_max_stale_generations(100)
        .with_target_fitness_score(100 * 100)
        .with_fitness(DistanceTo(0.5, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        // .with_mutate(MutateSingleGeneRandom::new(0.2))
        .with_mutate(MutateSingleGeneNeighbour::new(0.2))
        .with_crossover(CrossoverUniform::new(true))
        .with_compete(CompeteTournament::new(4))
        .with_reporter(EvolveReporterSimple::new(100))
        .call(&mut rng)
        .unwrap();

    let duration = now.elapsed();

    println!("{}", evolve);
    println!("duration: {:?}", duration);
}