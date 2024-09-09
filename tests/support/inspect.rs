use fixedbitset::Block;
use genetic_algorithm::chromosome::{BitChromosome, Chromosome, OwnesGenes};
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::{BitGenotype, Genotype};
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<C: OwnesGenes>(chromosome: &C) -> C::Genes {
    chromosome.genes().clone()
}
#[allow(dead_code)]
pub fn chromosome_to_str(chromosome: &BitChromosome) -> String {
    format!("{:b}", chromosome.genes)
}
#[allow(dead_code)]
pub fn chromosome_to_blocks(chromosome: &BitChromosome) -> &[Block] {
    chromosome.genes.as_slice()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<C: OwnesGenes>(
    chromosome: &C,
) -> (C::Genes, Option<FitnessValue>) {
    (chromosome.genes().clone(), chromosome.fitness_score())
}

#[allow(dead_code)]
pub fn chromosomes<C: OwnesGenes>(chromosomes: &Vec<C>) -> Vec<C::Genes> {
    chromosomes.iter().map(chromosome).collect()
}
#[allow(dead_code)]
pub fn chromosomes_to_str(chromosomes: &Vec<BitChromosome>) -> Vec<String> {
    chromosomes.iter().map(chromosome_to_str).collect()
}
#[allow(dead_code)]
pub fn chromosomes_to_blocks(chromosomes: &Vec<BitChromosome>) -> Vec<&[Block]> {
    chromosomes.iter().map(chromosome_to_blocks).collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_fitness_score<C: OwnesGenes>(
    chromosomes: &Vec<C>,
) -> Vec<(C::Genes, Option<FitnessValue>)> {
    chromosomes
        .iter()
        .map(chromosome_with_fitness_score)
        .collect()
}

#[allow(dead_code)]
pub fn population<G: Genotype>(population: &Population<G>) -> Vec<G::Genes>
where
    G::Chromosome: OwnesGenes<Genes = G::Genes>,
{
    population.chromosomes.iter().map(chromosome).collect()
}
#[allow(dead_code)]
pub fn population_to_str(population: &Population<BitGenotype>) -> Vec<String> {
    population
        .chromosomes
        .iter()
        .map(chromosome_to_str)
        .collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G: Genotype>(
    population: &Population<G>,
) -> Vec<(G::Genes, Option<FitnessValue>)>
where
    G::Chromosome: OwnesGenes<Genes = G::Genes>,
{
    population
        .chromosomes
        .iter()
        .map(chromosome_with_fitness_score)
        .collect()
}
