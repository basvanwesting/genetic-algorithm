use genetic_algorithm::centralized::chromosome::Chromosome;
use genetic_algorithm::centralized::fitness::FitnessValue;
use genetic_algorithm::centralized::genotype::Genotype;
use genetic_algorithm::centralized::population::Population;

#[allow(dead_code)]
pub fn chromosome<G>(genotype: &G, chromosome: &G::Chromosome) -> Vec<G::Allele>
where
    G: Genotype,
    G::Allele: Clone,
{
    genotype.genes_slice(chromosome).to_vec()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<G>(
    genotype: &G,
    chromosome: &G::Chromosome,
) -> (Vec<G::Allele>, Option<FitnessValue>)
where
    G: Genotype,
    G::Chromosome: Chromosome,
    G::Allele: Clone,
{
    (
        genotype.genes_slice(chromosome).to_vec(),
        chromosome.fitness_score(),
    )
}

#[allow(dead_code)]
pub fn chromosome_with_age<G>(genotype: &G, chromosome: &G::Chromosome) -> (Vec<G::Allele>, usize)
where
    G: Genotype,
    G::Chromosome: Chromosome,
    G::Allele: Clone,
{
    (genotype.genes_slice(chromosome).to_vec(), chromosome.age())
}

#[allow(dead_code)]
pub fn chromosomes<G>(genotype: &G, chromosomes: &[G::Chromosome]) -> Vec<Vec<G::Allele>>
where
    G: Genotype,
    G::Allele: Clone,
{
    chromosomes
        .iter()
        .map(|c| chromosome(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_fitness_score<G>(
    genotype: &G,
    chromosomes: &[G::Chromosome],
) -> Vec<(Vec<G::Allele>, Option<FitnessValue>)>
where
    G: Genotype,
    G::Chromosome: Chromosome,
    G::Allele: Clone,
{
    chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_age<G>(genotype: &G, chromosomes: &[G::Chromosome]) -> Vec<(Vec<G::Allele>, usize)>
where
    G: Genotype,
    G::Chromosome: Chromosome,
    G::Allele: Clone,
{
    chromosomes
        .iter()
        .map(|c| chromosome_with_age(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn population<G>(genotype: &G, population: &Population<G::Chromosome>) -> Vec<Vec<G::Allele>>
where
    G: Genotype,
    G::Allele: Clone,
{
    population
        .chromosomes
        .iter()
        .map(|c| chromosome(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<G>(
    genotype: &G,
    population: &Population<G::Chromosome>,
) -> Vec<(Vec<G::Allele>, Option<FitnessValue>)>
where
    G: Genotype,
    G::Chromosome: Chromosome,
    G::Allele: Clone,
{
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_with_fitness_score(genotype, c))
        .collect()
}

#[allow(dead_code)]
pub fn population_with_age<G>(
    genotype: &G,
    population: &Population<G::Chromosome>,
) -> Vec<(Vec<G::Allele>, usize)>
where
    G: Genotype,
    G::Chromosome: Chromosome,
    G::Allele: Clone,
{
    population
        .chromosomes
        .iter()
        .map(|c| chromosome_with_age(genotype, c))
        .collect()
}
