use crate::chromosome::Chromosome;
use crate::fitness;
use crate::fitness::{FitnessOrdering, FitnessValue};
use crate::genotype::{Genotype, MultiIndexGenotype};
use crate::meta::config::Config;
use crate::meta::stats::Stats;
use rand::prelude::*;
use rand::rngs::SmallRng;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Fitness<'a, G: Genotype, F: fitness::Fitness<Genotype = G>> {
    pub config: &'a Config<G, F>,
}
impl<'a, G: Genotype, F: fitness::Fitness<Genotype = G>> fitness::Fitness for Fitness<'a, G, F> {
    type Genotype = MultiIndexGenotype;
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let evolve_builder = self.config.evolve_builder_for_chromosome(chromosome);
        let mut stats = Stats::new();
        let mut rng = SmallRng::from_entropy();

        for _ in 0..self.config.rounds {
            let now = Instant::now();
            let evolve = evolve_builder.clone().build().unwrap().call(&mut rng);

            stats.durations.push(now.elapsed());
            stats.best_generations.push(evolve.best_generation);
            stats.best_fitness_scores.push(evolve.best_fitness_score());
        }
        println!(
            "population_size: {} | max_stale_generations: {:?} | target_fitness_score: {:?} | degeneration_range {:?} | mutate: {:?} | crossover: {:?} | compete: {:?}",
            evolve_builder.population_size,
            evolve_builder.max_stale_generations,
            evolve_builder.target_fitness_score,
            evolve_builder.degeneration_range,
            evolve_builder.mutate,
            evolve_builder.crossover,
            evolve_builder.compete
        );
        println!("  {}", stats);

        let mut score: FitnessValue = 0;
        match evolve_builder.fitness_ordering {
            FitnessOrdering::Maximize => {
                score += stats.best_fitness_score_mean() as FitnessValue
                    * self.config.evolve_fitness_to_micro_second_factor;
                score -= stats.duration_mean_subsec_micros() as FitnessValue;
            }
            FitnessOrdering::Minimize => {
                score += stats.best_fitness_score_mean() as FitnessValue
                    * self.config.evolve_fitness_to_micro_second_factor;
                score += stats.duration_mean_subsec_micros() as FitnessValue;
            }
        }
        Some(score)
    }
}
