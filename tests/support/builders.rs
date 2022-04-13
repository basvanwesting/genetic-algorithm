use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::gene::Gene;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn population_from_booleans(data: Vec<Vec<bool>>) -> Population {
    let chromosomes = data
        .into_iter()
        .map(|gene_values| chromosome_from_booleans(gene_values))
        .collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn chromosome_from_booleans(gene_values: Vec<bool>) -> Chromosome {
    let genes = gene_values.into_iter().map(|v| Gene(v)).collect();
    Chromosome::new(genes)
}

#[allow(dead_code)]
pub fn booleans_from_chromosome(chromosome: Chromosome) -> Vec<bool> {
    chromosome.genes.into_iter().map(|g| g.0).collect()
}

#[allow(dead_code)]
pub fn booleans_from_population(population: Population) -> Vec<Vec<bool>> {
    population
        .chromosomes
        .into_iter()
        .map(|c| booleans_from_chromosome(c))
        .collect()
}
