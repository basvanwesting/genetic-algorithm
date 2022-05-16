use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<T: Genotype>(genes: Vec<T::Allele>) -> Chromosome<T> {
    Chromosome::new(genes)
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<T: Genotype>(
    genes: Vec<T::Allele>,
    fitness_score: Option<FitnessValue>,
) -> Chromosome<T> {
    Chromosome {
        genes,
        fitness_score,
    }
}

#[allow(dead_code)]
pub fn population<T: Genotype>(data: Vec<Vec<T::Allele>>) -> Population<T> {
    let chromosomes = data.into_iter().map(|genes| chromosome(genes)).collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<T: Genotype>(
    data: Vec<(Vec<T::Allele>, Option<FitnessValue>)>,
) -> Population<T> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}
