use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<G: Genotype>(genes: G::Genes) -> Chromosome<G> {
    Chromosome::new(genes)
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<G: Genotype>(
    genes: G::Genes,
    fitness_score: Option<FitnessValue>,
) -> Chromosome<G> {
    Chromosome {
        genes,
        fitness_score,
        age: 0,
        reference_id: 0,
    }
}

#[allow(dead_code)]
pub fn population<G: Genotype>(data: Vec<G::Genes>) -> Population<G> {
    let chromosomes = data.into_iter().map(|genes| chromosome(genes)).collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G: Genotype>(
    data: Vec<(G::Genes, Option<FitnessValue>)>,
) -> Population<G> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}
