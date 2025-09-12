use genetic_algorithm::distributed::allele::Allele;
use genetic_algorithm::distributed::chromosome::Chromosome;
use genetic_algorithm::distributed::fitness::FitnessValue;
use genetic_algorithm::distributed::population::Population;

#[allow(dead_code)]
pub fn chromosome<T: Allele>(genes: Vec<T>) -> Chromosome<T> {
    let mut c = Chromosome::new(genes);
    c.reset_state();
    c
}
#[allow(dead_code)]
pub fn chromosome_with_fitness_score<T: Allele>(
    genes: Vec<T>,
    fitness_score: Option<FitnessValue>,
) -> Chromosome<T> {
    let mut chromosome = Chromosome::new(genes);
    chromosome.reset_state();
    chromosome.set_fitness_score(fitness_score);
    chromosome
}

#[allow(dead_code)]
pub fn chromosome_with_age<T: Allele>(genes: Vec<T>, age: usize) -> Chromosome<T> {
    let mut chromosome = Chromosome::new(genes);
    chromosome.reset_state();
    chromosome.set_age(age);
    chromosome
}

#[allow(dead_code)]
pub fn population<T: Allele>(data: Vec<Vec<T>>) -> Population<T> {
    let chromosomes = data.into_iter().map(chromosome).collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<T: Allele>(
    data: Vec<(Vec<T>, Option<FitnessValue>)>,
) -> Population<T> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_age<T: Allele>(data: Vec<(Vec<T>, usize)>) -> Population<T> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_age(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn chromosome_without_genes_hash<T: Allele>(genes: Vec<T>) -> Chromosome<T> {
    Chromosome::new(genes)
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score_without_genes_hash<T: Allele>(
    genes: Vec<T>,
    fitness_score: Option<FitnessValue>,
) -> Chromosome<T> {
    let mut chromosome = Chromosome::new(genes);
    chromosome.set_fitness_score(fitness_score);
    chromosome
}

#[allow(dead_code)]
pub fn population_without_genes_hash<T: Allele>(data: Vec<Vec<T>>) -> Population<T> {
    let chromosomes = data
        .into_iter()
        .map(chromosome_without_genes_hash)
        .collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores_without_genes_hash<T: Allele>(
    data: Vec<(Vec<T>, Option<FitnessValue>)>,
) -> Population<T> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score_without_genes_hash(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}
