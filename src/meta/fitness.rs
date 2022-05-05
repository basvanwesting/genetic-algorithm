use crate::chromosome::Chromosome;
use crate::evolve::Evolve;
use crate::fitness;
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::meta::{MetaConfig, MetaStats};
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Fitness<'a, G: Genotype, F: fitness::Fitness<Genotype = G>> {
    pub config: &'a MetaConfig<G, F>,
}
impl<'a, G: Genotype, F: fitness::Fitness<Genotype = G>> fitness::Fitness for Fitness<'a, G, F> {
    type Genotype = MultiIndexGenotype;
    fn call_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> isize {
        let genotype = self.config.evolve_genotype.clone();
        let fitness = self.config.evolve_fitness.clone();
        let evolve_config = self.config.evolve_config_for_chromosome(chromosome);

        let mut stats = MetaStats::new();
        for _ in 0..self.config.rounds {
            let rng = SmallRng::from_entropy();
            let now = Instant::now();

            let evolve = Evolve::new(genotype.clone(), rng)
                .with_fitness(fitness.clone())
                .with_population_size(evolve_config.population_size)
                .with_max_stale_generations_option(evolve_config.max_stale_generations_option)
                .with_target_fitness_score_option(evolve_config.target_fitness_score_option)
                .with_degeneration_range_option(evolve_config.degeneration_range_option.clone())
                .with_mutate(evolve_config.mutate.clone())
                .with_crossover(evolve_config.crossover.clone())
                .with_compete(evolve_config.compete.clone())
                .call();

            stats.durations.push(now.elapsed());
            stats.best_generations.push(evolve.best_generation);
            stats.best_fitness_scores.push(evolve.best_fitness_score());
        }
        println!(
            "population_size: {} | max_stale_generations: {:?} | target_fitness_score: {:?} | degeneration_range {:?} | mutate: {:?} | crossover: {:?} | compete: {:?}",
            evolve_config.population_size,
            evolve_config.max_stale_generations_option,
            evolve_config.target_fitness_score_option,
            evolve_config.degeneration_range_option,
            evolve_config.mutate,
            evolve_config.crossover,
            evolve_config.compete
        );
        println!("  {}", stats);

        let mut score: isize = 0;
        score += stats.best_fitness_score_mean() as isize * 1_000_000_000;
        score -= stats.duration_mean_subsec_micros() as isize;
        score
    }
}
