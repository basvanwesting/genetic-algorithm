use crate::chromosome::Chromosome;
use crate::compete::CompeteDispatch;
use crate::crossover::CrossoverDispatch;
use crate::evolve::Evolve;
use crate::evolve_stats::EvolveStats;
use crate::fitness::Fitness;
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::meta_config::MetaConfig;
use crate::mutate::MutateDispatch;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::ops::Range;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Meta<G: Genotype, F: Fitness<Genotype = G>> {
    pub config: MetaConfig<G, F>,
}
impl<G: Genotype, F: Fitness<Genotype = G>> Fitness for Meta<G, F> {
    type Genotype = MultiIndexGenotype;
    fn call_for_chromosome(&self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        let genotype = self.config.evolve_genotype.clone();
        let fitness = self.config.evolve_fitness.clone();

        let population_size = self.config.population_sizes[chromosome.genes[0]];
        let max_stale_generations_option =
            self.config.max_stale_generations_options[chromosome.genes[1]].clone();
        let target_fitness_score_option =
            self.config.target_fitness_score_options[chromosome.genes[2]].clone();
        let degeneration_range_option =
            self.config.degeneration_range_options[chromosome.genes[3]].clone();
        let mutate = self.config.mutates[chromosome.genes[4]].clone();
        let crossover = self.config.crossovers[chromosome.genes[5]].clone();
        let compete = self.config.competes[chromosome.genes[6]].clone();

        let mut stats = EvolveStats::new();
        for _ in 0..self.config.rounds {
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
