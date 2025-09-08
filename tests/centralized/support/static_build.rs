use genetic_algorithm::centralized::chromosome::{Chromosome, ChromosomeManager, GenesPointer};
use genetic_algorithm::centralized::fitness::FitnessValue;
use genetic_algorithm::centralized::genotype::Genotype;
use genetic_algorithm::centralized::population::Population;

#[allow(dead_code)]
pub fn chromosome<G, T, const N: usize>(genotype: &mut G, genes: Vec<T>) -> G::Chromosome
where
    G: Genotype<Allele = T, Genes = Box<[T; N]>> + ChromosomeManager<G>,
    G::Chromosome: GenesPointer,
    T: Clone,
{
    let boxed_genes: Box<[T; N]> = genes.into_boxed_slice().try_into().ok().unwrap();
    genotype.chromosome_constructor_genes(&boxed_genes)
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<G, T, const N: usize>(
    genotype: &mut G,
    genes: Vec<T>,
    fitness_score: Option<FitnessValue>,
) -> G::Chromosome
where
    G: Genotype<Allele = T, Genes = Box<[T; N]>> + ChromosomeManager<G>,
    G::Chromosome: GenesPointer + Chromosome,
    T: Clone,
{
    let mut chromosome = chromosome(genotype, genes);
    chromosome.set_fitness_score(fitness_score);
    chromosome
}

#[allow(dead_code)]
pub fn chromosome_with_age<G, T, const N: usize>(
    genotype: &mut G,
    genes: Vec<T>,
    age: usize,
) -> G::Chromosome
where
    G: Genotype<Allele = T, Genes = Box<[T; N]>> + ChromosomeManager<G>,
    G::Chromosome: GenesPointer + Chromosome,
    T: Clone,
{
    let mut chromosome = chromosome(genotype, genes);
    chromosome.set_age(age);
    chromosome
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G, T, const N: usize>(
    genotype: &mut G,
    genes_and_scores: Vec<(Vec<T>, Option<FitnessValue>)>,
) -> Population<G::Chromosome>
where
    G: Genotype<Allele = T, Genes = Box<[T; N]>> + ChromosomeManager<G>,
    G::Chromosome: GenesPointer + Chromosome,
    T: Clone,
{
    let chromosomes = genes_and_scores
        .into_iter()
        .map(|(genes, score)| chromosome_with_fitness_score(genotype, genes, score))
        .collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_age<G, T, const N: usize>(
    genotype: &mut G,
    genes_and_ages: Vec<(Vec<T>, usize)>,
) -> Population<G::Chromosome>
where
    G: Genotype<Allele = T, Genes = Box<[T; N]>> + ChromosomeManager<G>,
    G::Chromosome: GenesPointer + Chromosome,
    T: Clone,
{
    let chromosomes = genes_and_ages
        .into_iter()
        .map(|(genes, age)| chromosome_with_age(genotype, genes, age))
        .collect();
    Population::new(chromosomes)
}

