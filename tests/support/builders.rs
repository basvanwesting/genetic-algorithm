use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::gene::Gene;
use genetic_algorithm::population::Population;

pub fn build_population_from_booleans(data: Vec<Vec<bool>>) -> Population {
    let chromosomes = data
        .into_iter()
        .map(|gene_values| build_chromosome_from_booleans(gene_values))
        .collect();
    Population::new(chromosomes)
}

pub fn build_chromosome_from_booleans(gene_values: Vec<bool>) -> Chromosome {
    let genes = gene_values.into_iter().map(|v| Gene::new(v)).collect();
    Chromosome::new(genes)
}

pub fn build_booleans_from_chromosome(chromosome: Chromosome) -> Vec<bool> {
    chromosome.genes.into_iter().map(|g| g.value).collect()
}

pub fn build_booleans_from_population(population: Population) -> Vec<Vec<bool>> {
    population
        .chromosomes
        .into_iter()
        .map(|c| build_booleans_from_chromosome(c))
        .collect()
}
