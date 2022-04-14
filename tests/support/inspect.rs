use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::gene::Gene;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<T: Gene>(chromosome: &Chromosome<T>) -> Vec<T> {
    chromosome.genes.clone()
}

#[allow(dead_code)]
pub fn population<T: Gene>(population: &Population<T>) -> Vec<Vec<T>> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome(&c))
        .collect()
}
