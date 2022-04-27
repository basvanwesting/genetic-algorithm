use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::gene::Gene;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn population<T: Gene>(data: Vec<Vec<T>>) -> Population<T> {
    let chromosomes = data
        .into_iter()
        .map(|gene_values| chromosome(gene_values))
        .collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn chromosome<T: Gene>(gene_values: Vec<T>) -> Chromosome<T> {
    Chromosome::new(gene_values)
}
