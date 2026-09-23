#[cfg(test)]
use crate::support::*;
use genetic_algorithm::strategy::evolve::prelude::*;
use rayon::prelude::*;

// Fitness which uses rayon itself, so the par_fitness worker threads can steal other
// chromosomes of the population while waiting on the nested par_iter
#[derive(Clone, Debug)]
struct CountTrueNestedPar;
impl Fitness for CountTrueNestedPar {
    type Genotype = BinaryGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &FitnessChromosome<Self>,
        _genotype: &FitnessGenotype<Self>,
    ) -> Option<FitnessValue> {
        Some(chromosome.genes.par_iter().filter(|&value| *value).count() as FitnessValue)
    }
}

#[test]
fn par_fitness_with_nested_rayon_fitness() {
    let genotype = BinaryGenotype::builder()
        .with_genes_size(1000)
        .build()
        .unwrap();

    let evolve = Evolve::builder()
        .with_genotype(genotype)
        .with_target_population_size(2000)
        .with_max_generations(2)
        .with_mutate(MutateSingleGene::new(0.2))
        .with_fitness(CountTrueNestedPar)
        .with_par_fitness(true)
        .with_crossover(CrossoverUniform::new(0.7, 0.8))
        .with_select(SelectTournament::new(0.5, 0.02, 4))
        .with_rng_seed_from_u64(0)
        .call()
        .unwrap();

    assert!(evolve.best_fitness_score().is_some());
    assert!(evolve
        .state
        .population
        .chromosomes
        .iter()
        .all(|c| c.fitness_score().is_some()));
}
