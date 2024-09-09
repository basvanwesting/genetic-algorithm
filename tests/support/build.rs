use fixedbitset::Block;
use genetic_algorithm::chromosome::{BitChromosome, Chromosome, OwnesGenes};
use genetic_algorithm::fitness::FitnessValue;
use genetic_algorithm::genotype::{BitGenotype, Genotype};
use genetic_algorithm::population::Population;

#[allow(dead_code)]
pub fn chromosome<C: OwnesGenes>(genes: C::Genes) -> C {
    C::new(genes)
}
#[allow(dead_code)]
pub fn chromosome_from_str(str: &str) -> BitChromosome {
    BitChromosome::new(BitGenotype::genes_from_str(str))
}
#[allow(dead_code)]
pub fn chromosome_from_blocks<I: IntoIterator<Item = Block>>(
    bits: usize,
    blocks: I,
) -> BitChromosome {
    BitChromosome::new(BitGenotype::genes_from_blocks(bits, blocks))
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<C: OwnesGenes>(
    genes: C::Genes,
    fitness_score: Option<FitnessValue>,
) -> C {
    let mut chromosome = C::new(genes);
    chromosome.set_fitness_score(fitness_score);
    chromosome
}

#[allow(dead_code)]
pub fn population<G: Genotype>(data: Vec<G::Genes>) -> Population<G>
where
    G::Chromosome: OwnesGenes<Genes = G::Genes>,
{
    let chromosomes = data.into_iter().map(chromosome).collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G: Genotype>(
    data: Vec<(G::Genes, Option<FitnessValue>)>,
) -> Population<G>
where
    G::Chromosome: OwnesGenes<Genes = G::Genes>,
{
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}
