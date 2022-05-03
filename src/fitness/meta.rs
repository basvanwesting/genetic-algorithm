use crate::chromosome::Chromosome;
use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::evolve::Evolve;
use crate::evolve_stats::EvolveStats;
use crate::fitness::Fitness;
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::mutate::MutateDispatch;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::ops::Range;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Meta<G: Genotype, F: Fitness<Genotype = G>> {
    pub rounds: usize,
    pub evolve_genotype: G,
    pub evolve_fitness: F,
    pub population_sizes: Vec<usize>,
    pub max_stale_generations_options: Vec<Option<usize>>,
    pub target_fitness_score_options: Vec<Option<isize>>,
    pub degeneration_range_options: Vec<Option<Range<f32>>>,
    pub mutates: Vec<MutateDispatch>,
    pub crossovers: Vec<CrossoverDispatch>,
    pub competes: Vec<CompeteDispatch>,
}
impl<G: Genotype, F: Fitness<Genotype = G>> Fitness for Meta<G, F> {
    type Genotype = MultiIndexGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        let genotype = self.evolve_genotype.clone();
        let fitness = self.evolve_fitness.clone();

        let population_size = self.population_sizes[chromosome.genes[0]];
        let max_stale_generations_option =
            self.max_stale_generations_options[chromosome.genes[1]].clone();
        let target_fitness_score_option =
            self.target_fitness_score_options[chromosome.genes[2]].clone();
        let degeneration_range_option =
            self.degeneration_range_options[chromosome.genes[3]].clone();
        let mutate = self.mutates[chromosome.genes[4]].clone();
        let crossover = self.crossovers[chromosome.genes[5]].clone();
        let compete = self.competes[chromosome.genes[6]].clone();

        let mut stats = EvolveStats::new();
        for _ in 0..self.rounds {
            let rng = SmallRng::from_entropy();
            let now = Instant::now();

            let evolve = Evolve::new(genotype.clone(), rng)
                .with_population_size(population_size)
                .with_max_stale_generations_option(max_stale_generations_option.clone())
                .with_target_fitness_score_option(target_fitness_score_option.clone())
                .with_degeneration_range_option(degeneration_range_option.clone())
                .with_mutate(mutate.clone())
                .with_fitness(fitness.clone())
                .with_crossover(crossover.clone())
                .with_compete(compete.clone())
                .call();

            stats.durations.push(now.elapsed());
            stats.best_generations.push(evolve.best_generation);
            stats.best_fitness_scores.push(evolve.best_fitness_score());
        }
        println!(
            "population_size: {} | max_stale_generations: {:?} | target_fitness_score: {:?} | degeneration_range {:?} | mutate: {:?} | crossover: {:?} | compete: {:?}",
            population_size, max_stale_generations_option, target_fitness_score_option, degeneration_range_option, mutate, crossover, compete
        );
        println!("  {}", stats);

        let mut score: isize = 0;
        score += stats.best_fitness_score_mean() as isize * 1_000_000_000;
        score -= stats.duration_mean_subsec_micros() as isize;
        score
    }
}
