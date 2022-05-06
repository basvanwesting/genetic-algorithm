use crate::chromosome::Chromosome;
use crate::fitness;
use crate::fitness::FitnessValue;
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
    fn call_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue> {
        let evolve_config = self.config.evolve_config_for_chromosome(chromosome);
        let mut stats = MetaStats::new();
        let mut rng = SmallRng::from_entropy();

        for _ in 0..self.config.rounds {
            let now = Instant::now();
            let evolve = evolve_config.clone().build().call(&mut rng);

            stats.durations.push(now.elapsed());
            stats.best_generations.push(evolve.best_generation);
            stats.best_fitness_scores.push(evolve.best_fitness_score());
        }
        println!(
            "population_size: {} | max_stale_generations: {:?} | target_fitness_score: {:?} | degeneration_range {:?} | mutate: {:?} | crossover: {:?} | compete: {:?}",
            evolve_config.population_size,
            evolve_config.max_stale_generations,
            evolve_config.target_fitness_score,
            evolve_config.degeneration_range,
            evolve_config.mutate,
            evolve_config.crossover,
            evolve_config.compete
        );
        println!("  {}", stats);

        let mut score: FitnessValue = 0;
        score += stats.best_fitness_score_mean() as FitnessValue * 1_000_000_000;
        score -= stats.duration_mean_subsec_micros() as FitnessValue;
        Some(score)
    }
}
