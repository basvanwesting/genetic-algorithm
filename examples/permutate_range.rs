use genetic_algorithm::strategy::permutate::prelude::*;

#[derive(Clone, Debug)]
pub struct DistanceTo(pub f32, pub f32); // target, precision
impl Fitness for DistanceTo {
    type Genotype = RangeGenotype<f32>;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        Some(
            chromosome
                .genes
                .iter()
                .map(|v| (v - self.0).abs() / self.1)
                .sum::<f32>() as FitnessValue,
        )
    }
}

fn main() {
    env_logger::init();

    let genotype = RangeGenotype::builder()
        .with_genes_size(4)
        .with_allele_range(0.0..=1.0)
        .with_mutation_type(MutationType::StepScaled(vec![
            0.1, 0.01, 0.001, 0.0001, 0.00001,
        ]))
        .build()
        .unwrap();

    println!("{}", genotype);

    let permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(DistanceTo(0.55555, 1e-5))
        .with_fitness_ordering(FitnessOrdering::Minimize)
        .with_reporter(PermutateReporterSimple::new(10_000))
        .call()
        .unwrap();

    println!("{}", permutate);
}
