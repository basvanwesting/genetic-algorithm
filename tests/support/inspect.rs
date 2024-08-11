use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::Allele;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<T: Allele>(chromosome: &Chromosome<T>) -> Vec<T> {
    chromosome.genes.clone()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<T: Allele>(
    chromosome: &Chromosome<T>,
) -> (Vec<T>, Option<FitnessValue>) {
    (chromosome.genes.clone(), chromosome.fitness_score)
}

#[allow(dead_code)]
pub fn chromosomes<T: Allele>(chromosomes: &Vec<Chromosome<T>>) -> Vec<Vec<T>> {
    chromosomes.iter().map(|c| chromosome(&c)).collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_fitness_score<T: Allele>(
    chromosomes: &Vec<Chromosome<T>>,
) -> Vec<(Vec<T>, Option<FitnessValue>)> {
    chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(&c))
        .collect()
}

#[allow(dead_code)]
pub fn population<T: Allele>(population: &Population<T>) -> Vec<Vec<T>> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome(&c))
        .collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<T: Allele>(
    population: &Population<T>,
) -> Vec<(Vec<T>, Option<FitnessValue>)> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(&c))
        .collect()
}
