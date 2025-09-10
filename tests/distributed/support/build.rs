use genetic_algorithm::distributed::chromosome::GenesOwner;
use genetic_algorithm::distributed::fitness::FitnessValue;
use genetic_algorithm::distributed::population::Population;

#[allow(dead_code)]
pub fn chromosome<C: GenesOwner>(genes: C::Genes) -> C {
    C::new(genes)
}
#[allow(dead_code)]
pub fn chromosome_with_fitness_score<C: GenesOwner>(
    genes: C::Genes,
    fitness_score: Option<FitnessValue>,
) -> C {
    let mut chromosome = C::new(genes);
    chromosome.set_fitness_score(fitness_score);
    chromosome
}

#[allow(dead_code)]
pub fn chromosome_with_age<C: GenesOwner>(genes: C::Genes, age: usize) -> C {
    let mut chromosome = C::new(genes);
    chromosome.set_age(age);
    chromosome
}

#[allow(dead_code)]
pub fn population<C: GenesOwner>(data: Vec<C::Genes>) -> Population<C> {
    let chromosomes = data.into_iter().map(chromosome).collect();
    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_fitness_scores<C: GenesOwner>(
    data: Vec<(C::Genes, Option<FitnessValue>)>,
) -> Population<C> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_fitness_score(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}

#[allow(dead_code)]
pub fn population_with_age<C: GenesOwner>(data: Vec<(C::Genes, usize)>) -> Population<C> {
    let chromosomes = data
        .into_iter()
        .map(|tuple| chromosome_with_age(tuple.0, tuple.1))
        .collect();

    Population::new(chromosomes)
}
