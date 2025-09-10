use genetic_algorithm::distributed::chromosome::{Chromosome, GenesOwner};
use genetic_algorithm::distributed::fitness::FitnessValue;
use genetic_algorithm::distributed::population::Population;

#[allow(dead_code)]
pub fn chromosome<C: GenesOwner + Chromosome>(genes: C::Genes) -> C {
    let mut c = C::new(genes);
    c.update_state();
    c
}
#[allow(dead_code)]
pub fn chromosome_with_fitness_score<C: GenesOwner + Chromosome>(
    genes: C::Genes,
    fitness_score: Option<FitnessValue>,
) -> C {
    let mut chromosome = C::new(genes);
    chromosome.update_state();
    chromosome.set_fitness_score(fitness_score);
    chromosome
}

#[allow(dead_code)]
pub fn chromosome_with_age<C: GenesOwner + Chromosome>(genes: C::Genes, age: usize) -> C {
    let mut chromosome = C::new(genes);
    chromosome.update_state();
    chromosome.set_age(age);
    chromosome
}

#[allow(dead_code)]
pub fn population<C: GenesOwner + Chromosome>(data: Vec<C::Genes>) -> Population<C> {
    let chromosomes = data.into_iter().map(chromosome).collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<C: GenesOwner + Chromosome>(
    data: Vec<(C::Genes, Option<FitnessValue>)>,
) -> Population<C> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_age<C: GenesOwner + Chromosome>(
    data: Vec<(C::Genes, usize)>,
) -> Population<C> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_age(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn chromosome_without_genes_hash<C: GenesOwner>(genes: C::Genes) -> C {
    C::new(genes)
}

#[allow(dead_code)]
pub fn chromosome_with_fitness_score_without_genes_hash<C: GenesOwner + Chromosome>(
    genes: C::Genes,
    fitness_score: Option<FitnessValue>,
) -> C {
    let mut chromosome = C::new(genes);
    chromosome.set_fitness_score(fitness_score);
    chromosome
}

#[allow(dead_code)]
pub fn population_without_genes_hash<C: GenesOwner>(data: Vec<C::Genes>) -> Population<C> {
    let chromosomes = data
        .into_iter()
        .map(chromosome_without_genes_hash)
        .collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores_without_genes_hash<C: GenesOwner + Chromosome>(
    data: Vec<(C::Genes, Option<FitnessValue>)>,
) -> Population<C> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score_without_genes_hash(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}
