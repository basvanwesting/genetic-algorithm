use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn number_of_true_values_in_population(population: &Population) -> usize {
    population
        .chromosomes
        .iter()
        .map(|c| number_of_true_values_in_chromosome(&c))
        .sum()
}

#[allow(dead_code)]
pub fn number_of_true_values_in_chromosome(chromosome: &Chromosome) -> usize {
    chromosome.genes.iter().filter(|&gene| gene.0).count()
}

#[allow(dead_code)]
pub fn number_of_false_values_in_population(population: &Population) -> usize {
    population
        .chromosomes
        .iter()
        .map(|c| number_of_false_values_in_chromosome(&c))
        .sum()
}

#[allow(dead_code)]
pub fn number_of_false_values_in_chromosome(chromosome: &Chromosome) -> usize {
    chromosome.genes.iter().filter(|&gene| !gene.0).count()
}
