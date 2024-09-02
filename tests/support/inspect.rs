use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<G: Genotype>(chromosome: &Chromosome<G>) -> Vec<G::Allele> {
    chromosome.genes.clone()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<G: Genotype>(
    chromosome: &Chromosome<G>,
) -> (Vec<G::Allele>, Option<FitnessValue>) {
    (chromosome.genes.clone(), chromosome.fitness_score)
}

#[allow(dead_code)]
pub fn chromosomes<G: Genotype>(chromosomes: &Vec<Chromosome<G>>) -> Vec<Vec<G::Allele>> {
    chromosomes.iter().map(|c| chromosome(&c)).collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_fitness_score<G: Genotype>(
    chromosomes: &Vec<Chromosome<G>>,
) -> Vec<(Vec<G::Allele>, Option<FitnessValue>)> {
    chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(&c))
        .collect()
}

#[allow(dead_code)]
pub fn population<G: Genotype>(population: &Population<G>) -> Vec<Vec<G::Allele>> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome(&c))
        .collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G: Genotype>(
    population: &Population<G>,
) -> Vec<(Vec<G::Allele>, Option<FitnessValue>)> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(&c))
        .collect()
}
