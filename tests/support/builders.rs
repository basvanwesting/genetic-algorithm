use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::gene::BinaryGene;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn population_from_booleans(data: Vec<Vec<BinaryGene>>) -> Population<BinaryGene> {
    let chromosomes = data
        .into_iter()
        .map(|gene_values| chromosome_from_booleans(gene_values))
        .collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn chromosome_from_booleans(gene_values: Vec<BinaryGene>) -> Chromosome<BinaryGene> {
    Chromosome::new(gene_values)
}

#[allow(dead_code)]
pub fn booleans_from_chromosome(chromosome: Chromosome<BinaryGene>) -> Vec<BinaryGene> {
    chromosome.genes
}

#[allow(dead_code)]
pub fn booleans_from_population(population: Population<BinaryGene>) -> Vec<Vec<BinaryGene>> {
    population
        .chromosomes
        .into_iter()
        .map(|c| booleans_from_chromosome(c))
        .collect()
}
