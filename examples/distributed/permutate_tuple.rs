use genetic_algorithm::distributed::strategy::permutate::prelude::*;

type Item = (u8, u16, i8);

#[derive(Clone, Debug)]
struct TupleFitness;
impl Fitness for TupleFitness {
    type Genotype = ListGenotype<Item>;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        let score: isize = chromosome
            .genes
            .iter()
            .flat_map(|tuple| [tuple.0 as isize, tuple.1 as isize, tuple.2 as isize])
            .sum();
        Some(score)
    }
}

fn main() {
    env_logger::init();

    let genotype = ListGenotype::builder()
        .with_genes_size(7)
        .with_allele_list(vec![
            (1, 100, -10),
            (2, 200, -20),
            (3, 300, -30),
            (4, 400, -40),
            (5, 500, -50),
            (6, 600, -60),
            (7, 700, -70),
            (8, 800, -80),
        ])
        .build()
        .unwrap();

    println!("{}", genotype);

    let mut permutate = Permutate::builder()
        .with_genotype(genotype)
        .with_fitness(TupleFitness)
        // .with_par_fitness(true) // worse performance
        .with_reporter(PermutateReporterSimple::new(100_000))
        .build()
        .unwrap();

    permutate.call();
    println!("{}", permutate);
}
