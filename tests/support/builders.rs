use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::gene::{BinaryGene, ContinuousGene, DiscreteGene};
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn population_binary(data: Vec<Vec<BinaryGene>>) -> Population<BinaryGene> {
    let chromosomes = data
        .into_iter()
        .map(|gene_values| chromosome_binary(gene_values))
        .collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn chromosome_binary(gene_values: Vec<BinaryGene>) -> Chromosome<BinaryGene> {
    Chromosome::new(gene_values)
}

#[allow(dead_code)]
pub fn chromosome_discrete(gene_values: Vec<DiscreteGene>) -> Chromosome<DiscreteGene> {
    Chromosome::new(gene_values)
}

#[allow(dead_code)]
pub fn chromosome_continuous(gene_values: Vec<ContinuousGene>) -> Chromosome<ContinuousGene> {
    Chromosome::new(gene_values)
}

#[allow(dead_code)]
pub fn inspect_chromosome_binary(chromosome: Chromosome<BinaryGene>) -> Vec<BinaryGene> {
    chromosome.genes
}

#[allow(dead_code)]
pub fn inspect_population_binary(population: Population<BinaryGene>) -> Vec<Vec<BinaryGene>> {
    population
        .chromosomes
        .into_iter()
        .map(|c| inspect_chromosome_binary(c))
        .collect()
}
