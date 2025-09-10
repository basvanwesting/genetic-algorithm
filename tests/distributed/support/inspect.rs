use genetic_algorithm::distributed::chromosome::GenesOwner;
use genetic_algorithm::distributed::fitness::FitnessValue;
use genetic_algorithm::distributed::population::Population;

#[allow(dead_code)]
pub fn chromosome<C: GenesOwner>(chromosome: &C) -> C::Genes {
    chromosome.genes().clone()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<C: GenesOwner>(
    chromosome: &C,
) -> (C::Genes, Option<FitnessValue>) {
    (chromosome.genes().clone(), chromosome.fitness_score())
}
#[allow(dead_code)]
pub fn chromosome_with_age<C: GenesOwner>(chromosome: &C) -> (C::Genes, usize) {
    (chromosome.genes().clone(), chromosome.age())
}

#[allow(dead_code)]
pub fn chromosomes<C: GenesOwner>(chromosomes: &[C]) -> Vec<C::Genes> {
    chromosomes.iter().map(chromosome).collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_fitness_score<C: GenesOwner>(
    chromosomes: &[C],
) -> Vec<(C::Genes, Option<FitnessValue>)> {
    chromosomes
        .iter()
        .map(chromosome_with_fitness_score)
        .collect()
}
#[allow(dead_code)]
pub fn chromosomes_with_age<C: GenesOwner>(chromosomes: &[C]) -> Vec<(C::Genes, usize)> {
    chromosomes.iter().map(chromosome_with_age).collect()
}

#[allow(dead_code)]
pub fn population<C: GenesOwner>(population: &Population<C>) -> Vec<C::Genes> {
    population.chromosomes.iter().map(chromosome).collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<C: GenesOwner>(
    population: &Population<C>,
) -> Vec<(C::Genes, Option<FitnessValue>)> {
    population
        .chromosomes
        .iter()
        .map(chromosome_with_fitness_score)
        .collect()
}
#[allow(dead_code)]
pub fn population_with_age<C: GenesOwner>(population: &Population<C>) -> Vec<(C::Genes, usize)> {
    population
        .chromosomes
        .iter()
        .map(chromosome_with_age)
        .collect()
}
