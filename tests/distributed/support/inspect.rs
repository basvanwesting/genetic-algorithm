use genetic_algorithm::distributed::allele::Allele;
use genetic_algorithm::distributed::chromosome::Chromosome;
use genetic_algorithm::distributed::fitness::FitnessValue;
use genetic_algorithm::distributed::population::Population;

#[allow(dead_code)]
pub fn chromosome<T>(chromosome: &Chromosome<T>) -> Vec<T>
where
    T: Clone + Allele,
{
    chromosome.genes.clone()
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score<T>(
    chromosome: &Chromosome<T>,
) -> (Vec<T>, Option<FitnessValue>)
where
    T: Clone + Allele,
{
    (chromosome.genes.clone(), chromosome.fitness_score())
}
#[allow(dead_code)]
pub fn chromosome_with_age<T>(chromosome: &Chromosome<T>) -> (Vec<T>, usize)
where
    T: Clone + Allele,
{
    (chromosome.genes.clone(), chromosome.age())
}

#[allow(dead_code)]
pub fn chromosomes<T>(chromosomes: &[Chromosome<T>]) -> Vec<Vec<T>>
where
    T: Clone + Allele,
{
    chromosomes.iter().map(chromosome).collect()
}

#[allow(dead_code)]
pub fn chromosomes_with_fitness_score<T>(
    chromosomes: &[Chromosome<T>],
) -> Vec<(Vec<T>, Option<FitnessValue>)>
where
    T: Clone + Allele,
{
    chromosomes
        .iter()
        .map(chromosome_with_fitness_score)
        .collect()
}
#[allow(dead_code)]
pub fn chromosomes_with_age<T>(chromosomes: &[Chromosome<T>]) -> Vec<(Vec<T>, usize)>
where
    T: Clone + Allele,
{
    chromosomes.iter().map(chromosome_with_age).collect()
}

#[allow(dead_code)]
pub fn population<T>(population: &Population<T>) -> Vec<Vec<T>>
where
    T: Clone + Allele,
{
    population.chromosomes.iter().map(chromosome).collect()
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<T>(
    population: &Population<T>,
) -> Vec<(Vec<T>, Option<FitnessValue>)>
where
    T: Clone + Allele,
{
    population
        .chromosomes
        .iter()
        .map(chromosome_with_fitness_score)
        .collect()
}
#[allow(dead_code)]
pub fn population_with_age<T>(population: &Population<T>) -> Vec<(Vec<T>, usize)>
where
    T: Clone + Allele,
{
    population
        .chromosomes
        .iter()
        .map(chromosome_with_age)
        .collect()
}
