use crate::chromosome::Chromosome;
use crate::fitness;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::{Genotype, MultiDiscreteGenotype};
use crate::meta::config::Config;
use crate::meta::stats::Stats;
use crate::strategy::Strategy;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Fitness<'a, G: Genotype + Sync, F: fitness::Fitness<Genotype = G> + Sync> {
    pub config: &'a Config<G, F>,
}
impl<'a, G: Genotype + Sync, F: fitness::Fitness<Genotype = G> + Sync> fitness::Fitness
    for Fitness<'a, G, F>
{
    type Genotype = MultiDiscreteGenotype;
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let evolve_builder = self.config.evolve_builder_for_chromosome(chromosome);
        let mut stats = Stats::new();
        let mut rng = SmallRng::from_entropy();

        log::info!(
            "target-pop-size: {} | max-stale-gen: {:?} | max-age: {:?} | target-fitness: {:?} | {:?} | {:?} | {:?} | {:?}",
            evolve_builder.target_population_size,
            evolve_builder.max_stale_generations,
            evolve_builder.max_chromosome_age,
            evolve_builder.target_fitness_score,
            evolve_builder.mutate,
            evolve_builder.crossover,
            evolve_builder.compete,
            evolve_builder.extension,
        );
        for _ in 0..self.config.rounds {
            let now = Instant::now();
            let mut evolve = evolve_builder.clone().build().unwrap();
            evolve.call(&mut rng);

            stats.durations.push(now.elapsed());
            stats.best_generations.push(evolve.best_generation());
            stats.best_fitness_scores.push(evolve.best_fitness_score());
        }
        log::info!("  {}", stats);

        let mut score: FitnessValue = 0;
        score += stats.best_fitness_score_mean() as FitnessValue
            * self.config.evolve_fitness_to_micro_second_factor;
        match evolve_builder.fitness_ordering {
            FitnessOrdering::Maximize => {
                score -= stats.duration_mean_subsec_micros() as FitnessValue;
            }
            FitnessOrdering::Minimize => {
                score += stats.duration_mean_subsec_micros() as FitnessValue;
            }
        }
        Some(score)
    }
}
