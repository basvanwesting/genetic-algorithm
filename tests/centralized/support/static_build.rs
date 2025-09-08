use genetic_algorithm::centralized::chromosome::{ChromosomeManager, StaticBinaryChromosome};
use genetic_algorithm::centralized::genotype::StaticBinaryGenotype;

/// Build a StaticBinaryChromosome with the given genes
/// This helper allows clean test initialization similar to GenesOwner chromosomes
#[allow(dead_code)]
pub fn chromosome<const N: usize, const M: usize>(
    genotype: &mut StaticBinaryGenotype<N, M>,
    genes: Vec<bool>,
) -> StaticBinaryChromosome {
    // Create a Box<[bool; N]> from the Vec
    let mut genes_array = [false; N];
    for (i, &value) in genes.iter().enumerate().take(N) {
        genes_array[i] = value;
    }
    let boxed_genes = Box::new(genes_array);
    
    // Use the genotype's chromosome constructor
    genotype.chromosome_constructor_genes(&boxed_genes)
}

/// Build a StaticBinaryChromosome with all genes set to the same value
#[allow(dead_code)]
pub fn chromosome_with_value<const N: usize, const M: usize>(
    genotype: &mut StaticBinaryGenotype<N, M>,
    value: bool,
    size: usize,
) -> StaticBinaryChromosome {
    chromosome(genotype, vec![value; size])
}