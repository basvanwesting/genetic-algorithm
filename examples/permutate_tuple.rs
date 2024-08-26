use genetic_algorithm::strategy::permutate::prelude::*;

type Item = (u8, u16, i8);

#[derive(Clone, Debug)]
struct TupleFitness;
impl Fitness for TupleFitness {
    type Allele = Item;

    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Allele>,
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
        .with_genes_size(6)
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
        .build()
        .unwrap();

    let now = std::time::Instant::now();
    permutate.call();
    let duration = now.elapsed();

    println!("{}", permutate);
    println!("{:?}", duration);
}
