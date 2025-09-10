use genetic_algorithm::centralized::chromosome::{Chromosome, ChromosomeManager};
use genetic_algorithm::centralized::fitness::FitnessValue;
use genetic_algorithm::centralized::genotype::Genotype;
use genetic_algorithm::centralized::population::Population;

#[allow(dead_code)]
pub fn chromosome<G, T>(genotype: &mut G, genes: Vec<T>) -> Chromosome
where
    G: Genotype<Allele = T, Genes = Vec<T>> + ChromosomeManager<G>,
    T: Clone,
{
    genotype.chromosome_constructor_genes(&genes)
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<G, T>(
    genotype: &mut G,
    genes: Vec<T>,
    fitness_score: Option<FitnessValue>,
) -> Chromosome
where
    G: Genotype<Allele = T, Genes = Vec<T>> + ChromosomeManager<G>,
    T: Clone,
{
    let mut chromosome = chromosome(genotype, genes);
    chromosome.set_fitness_score(fitness_score);
    chromosome
}

#[allow(dead_code)]
pub fn chromosome_with_age<G, T>(genotype: &mut G, genes: Vec<T>, age: usize) -> Chromosome
where
    G: Genotype<Allele = T, Genes = Vec<T>> + ChromosomeManager<G>,
    T: Clone,
{
    let mut chromosome = chromosome(genotype, genes);
    chromosome.set_age(age);
    chromosome
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G, T>(
    genotype: &mut G,
    genes_and_scores: Vec<(Vec<T>, Option<FitnessValue>)>,
) -> Population
where
    G: Genotype<Allele = T, Genes = Vec<T>> + ChromosomeManager<G>,
    T: Clone,
{
    let chromosomes = genes_and_scores
        .into_iter()
        .map(|(genes, score)| chromosome_with_fitness_score(genotype, genes, score))
        .collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_age<G, T>(
    genotype: &mut G,
    genes_and_ages: Vec<(Vec<T>, usize)>,
) -> Population
where
    G: Genotype<Allele = T, Genes = Vec<T>> + ChromosomeManager<G>,
    T: Clone,
{
    let chromosomes = genes_and_ages
        .into_iter()
        .map(|(genes, age)| chromosome_with_age(genotype, genes, age))
        .collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population<G, T>(genotype: &mut G, data: Vec<Vec<T>>) -> Population
where
    G: Genotype<Allele = T, Genes = Vec<T>> + ChromosomeManager<G>,
    T: Clone,
{
    let chromosomes = data
        .into_iter()
        .map(|genes| chromosome(genotype, genes))
        .collect();
    Population::new(chromosomes)
}
