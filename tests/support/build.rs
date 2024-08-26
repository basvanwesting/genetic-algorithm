use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::Allele;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<T: Allele>(genes: Vec<T>) -> Chromosome<T> {
    Chromosome::new(genes)
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<T: Allele>(
    genes: Vec<T>,
    fitness_score: Option<FitnessValue>,
) -> Chromosome<T> {
    Chromosome {
        genes,
        fitness_score,
        age: 0,
        reference_id: 0,
    }
}

#[allow(dead_code)]
pub fn population<T: Allele>(data: Vec<Vec<T>>) -> Population<T> {
    let chromosomes = data.into_iter().map(|genes| chromosome(genes)).collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<T: Allele>(
    data: Vec<(Vec<T>, Option<FitnessValue>)>,
) -> Population<T> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}
