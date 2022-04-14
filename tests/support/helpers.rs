use genetic_algorithm::chromosome::Chromosome;
use genetic_algorithm::gene::BinaryGene;
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn number_of_true_values_in_population(population: &Population<BinaryGene>) -> usize {
    population
        .chromosomes
        .iter()
        .map(|c| number_of_true_values_in_chromosome(&c))
        .sum()
}

#[allow(dead_code)]
pub fn number_of_true_values_in_chromosome(chromosome: &Chromosome<BinaryGene>) -> usize {
    chromosome.genes.iter().filter(|&gene| *gene).count()
}

#[allow(dead_code)]
pub fn number_of_false_values_in_population(population: &Population<BinaryGene>) -> usize {
    population
        .chromosomes
        .iter()
        .map(|c| number_of_false_values_in_chromosome(&c))
        .sum()
}

#[allow(dead_code)]
pub fn number_of_false_values_in_chromosome(chromosome: &Chromosome<BinaryGene>) -> usize {
    chromosome.genes.iter().filter(|&gene| !*gene).count()
}
