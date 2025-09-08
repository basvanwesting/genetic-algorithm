use genetic_algorithm::centralized::chromosome::{Chromosome, GenesPointer};
use genetic_algorithm::centralized::fitness::FitnessValue;
use genetic_algorithm::centralized::genotype::Genotype;
use genetic_algorithm::centralized::population::Population;

#[allow(dead_code)]
pub fn chromosome<G, C>(genotype: &G, chromosome: &C) -> Vec<G::Allele>
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer,
    G::Allele: Clone,
{
    genotype.genes_slice(chromosome).to_vec()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<G, C>(
    genotype: &G,
    chromosome: &C,
) -> (Vec<G::Allele>, Option<FitnessValue>)
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer + Chromosome,
    G::Allele: Clone,
{
    (
        genotype.genes_slice(chromosome).to_vec(),
        chromosome.fitness_score(),
    )
}

#[allow(dead_code)]
pub fn chromosome_with_age<G, C>(genotype: &G, chromosome: &C) -> (Vec<G::Allele>, usize)
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer + Chromosome,
    G::Allele: Clone,
{
    (genotype.genes_slice(chromosome).to_vec(), chromosome.age())
}

#[allow(dead_code)]
pub fn chromosomes<G, C>(genotype: &G, chromosomes: &[C]) -> Vec<Vec<G::Allele>>
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer,
    G::Allele: Clone,
{
    chromosomes
        .iter()
        .map(|c| chromosome(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_fitness_score<G, C>(
    genotype: &G,
    chromosomes: &[C],
) -> Vec<(Vec<G::Allele>, Option<FitnessValue>)>
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer + Chromosome,
    G::Allele: Clone,
{
    chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_age<G, C>(genotype: &G, chromosomes: &[C]) -> Vec<(Vec<G::Allele>, usize)>
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer + Chromosome,
    G::Allele: Clone,
{
    chromosomes
        .iter()
        .map(|c| chromosome_with_age(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn population<G, C>(genotype: &G, population: &Population<C>) -> Vec<Vec<G::Allele>>
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer,
    G::Allele: Clone,
{
    population
        .chromosomes
        .iter()
        .map(|c| chromosome(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G, C>(
    genotype: &G,
    population: &Population<C>,
) -> Vec<(Vec<G::Allele>, Option<FitnessValue>)>
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer + Chromosome,
    G::Allele: Clone,
{
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn population_with_age<G, C>(
    genotype: &G,
    population: &Population<C>,
) -> Vec<(Vec<G::Allele>, usize)>
where
    G: Genotype<Chromosome = C>,
    C: GenesPointer + Chromosome,
    G::Allele: Clone,
{
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_with_age(genotype, c))
        .collect()
}

