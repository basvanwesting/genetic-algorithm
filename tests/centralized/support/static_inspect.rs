use genetic_algorithm::centralized::chromosome::StaticBinaryChromosome;
use genetic_algorithm::centralized::genotype::{Genotype, StaticBinaryGenotype};
use genetic_algorithm::centralized::population::Population;

/// Extract genes from a StaticBinaryChromosome by looking them up in the genotype's matrix
#[allow(dead_code)]
pub fn chromosome<const N: usize, const M: usize>(
    genotype: &StaticBinaryGenotype<N, M>,
    chromosome: &StaticBinaryChromosome,
) -> Vec<bool> {
    genotype.genes_slice(chromosome).to_vec()
}

/// Extract all genes from a Population of StaticBinaryChromosomes
#[allow(dead_code)]
pub fn chromosomes<const N: usize, const M: usize>(
    genotype: &StaticBinaryGenotype<N, M>,
    chromosomes: &[StaticBinaryChromosome],
) -> Vec<Vec<bool>> {
    chromosomes
        .iter()
        .map(|c| chromosome(genotype, c))
        .collect()
}

/// Extract all genes from a Population of StaticBinaryChromosomes
#[allow(dead_code)]
pub fn population<const N: usize, const M: usize>(
    genotype: &StaticBinaryGenotype<N, M>,
    population: &Population<StaticBinaryChromosome>,
) -> Vec<Vec<bool>> {
    population
        .chromosomes
        .iter()
        .map(|c| chromosome(genotype, c))
        .collect()
}

