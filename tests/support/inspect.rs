use fixedbitset::Block;
use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::{BitGenotype, Genotype};
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<G: Genotype>(chromosome: &Chromosome<G>) -> G::Genes {
    chromosome.genes.clone()
}
#[allow(dead_code)]
pub fn chromosome_to_str(chromosome: &Chromosome<BitGenotype>) -> String {
    format!("{:b}", chromosome.genes)
}
#[allow(dead_code)]
pub fn chromosome_to_blocks(chromosome: &Chromosome<BitGenotype>) -> &[Block] {
    chromosome.genes.as_slice()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<G: Genotype>(
    chromosome: &Chromosome<G>,
) -> (G::Genes, Option<FitnessValue>) {
    (chromosome.genes.clone(), chromosome.fitness_score)
}

#[allow(dead_code)]
pub fn chromosomes<G: Genotype>(chromosomes: &Vec<Chromosome<G>>) -> Vec<G::Genes> {
    chromosomes.iter().map(|c| chromosome(&c)).collect()
}
#[allow(dead_code)]
pub fn chromosomes_to_str(chromosomes: &Vec<Chromosome<BitGenotype>>) -> Vec<String> {
    chromosomes.iter().map(|c| chromosome_to_str(&c)).collect()
}
#[allow(dead_code)]
pub fn chromosomes_to_blocks(chromosomes: &Vec<Chromosome<BitGenotype>>) -> Vec<&[Block]> {
    chromosomes
        .iter()
        .map(|c| chromosome_to_blocks(&c))
        .collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_fitness_score<G: Genotype>(
    chromosomes: &Vec<Chromosome<G>>,
) -> Vec<(G::Genes, Option<FitnessValue>)> {
    chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(&c))
        .collect()
}

#[allow(dead_code)]
pub fn population<G: Genotype>(population: &Population<G>) -> Vec<G::Genes> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome(&c))
        .collect()
}
#[allow(dead_code)]
pub fn population_to_str(population: &Population<BitGenotype>) -> Vec<String> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_to_str(&c))
        .collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G: Genotype>(
    population: &Population<G>,
) -> Vec<(G::Genes, Option<FitnessValue>)> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(&c))
        .collect()
}
