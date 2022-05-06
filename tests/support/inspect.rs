use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<T: Genotype>(chromosome: &Chromosome<T>) -> Vec<T::Gene> {
    chromosome.genes.clone()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<T: Genotype>(
    chromosome: &Chromosome<T>,
) -> (Vec<T::Gene>, Option<FitnessValue>) {
    (chromosome.genes.clone(), chromosome.fitness_score)
}

#[allow(dead_code)]
pub fn population<T: Genotype>(population: &Population<T>) -> Vec<Vec<T::Gene>> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome(&c))
        .collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<T: Genotype>(
    population: &Population<T>,
) -> Vec<(Vec<T::Gene>, Option<FitnessValue>)> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(&c))
        .collect()
}
