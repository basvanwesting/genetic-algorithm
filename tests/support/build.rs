use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::genotype::Genotype;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn population<T: Genotype>(data: Vec<Vec<T::Gene>>) -> Population<T> {
    let chromosomes = data
        .into_iter()
        .map(|gene_values| chromosome(gene_values))
        .collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn chromosome<T: Genotype>(gene_values: Vec<T::Gene>) -> Chromosome<T> {
    Chromosome::new(gene_values)
}
