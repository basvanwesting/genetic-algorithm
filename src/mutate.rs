//! The mutation strategy, very important for avoiding local optimum lock-in. But don't overdo it,
//! as it degenerates the population too much if overused. Use a mutation probability generally between
//! 5% and 20%.
//mod dispatch;
//mod dynamic_once;
//mod dynamic_rounds;
//mod once;
//mod twice;

//pub use self::dispatch::Dispatch as MutateDispatch;
//pub use self::dynamic_once::DynamicOnce as MutateDynamicOnce;
//pub use self::dynamic_rounds::DynamicRounds as MutateDynamicRounds;
//pub use self::once::Once as MutateOnce;
//pub use self::twice::Twice as MutateTwice;

//pub use self::Mutate::DynamicOnce as MutateDynamicOnce;
//pub use self::Mutate::DynamicRounds as MutateDynamicRounds;
//pub use self::Mutate::Once as MutateOnce;
//pub use self::Mutate::Twice as MutateTwice;

use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Mutate {
    Once {
        mutation_probability: f32,
    },
    Twice {
        mutation_probability: f32,
    },
    DynamicOnce {
        mutation_probability: f32,
        mutation_probability_step: f32,
        target_uniformity: f32,
    },
    DynamicRounds {
        mutation_probability: f32,
        number_of_rounds: usize,
        target_uniformity: f32,
    },
}

impl Mutate {
    pub fn call<T: Genotype, R: Rng>(
        &mut self,
        genotype: &T,
        population: &mut Population<T>,
        rng: &mut R,
    ) {
        match self {
            Mutate::Once {
                mutation_probability,
            } => {
                let bool_sampler = Bernoulli::new(*mutation_probability as f64).unwrap();
                for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
                    if bool_sampler.sample(rng) {
                        genotype.mutate_chromosome_random(chromosome, rng);
                    }
                }
            }
            Mutate::Twice {
                mutation_probability,
            } => {
                let bool_sampler = Bernoulli::new(*mutation_probability as f64).unwrap();
                for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
                    if bool_sampler.sample(rng) {
                        genotype.mutate_chromosome_random(chromosome, rng);
                        genotype.mutate_chromosome_random(chromosome, rng);
                    }
                }
            }
            Mutate::DynamicOnce {
                mutation_probability,
                mutation_probability_step,
                target_uniformity,
            } => {
                if population.fitness_score_uniformity() > *target_uniformity {
                    *mutation_probability += *mutation_probability_step;
                } else if mutation_probability > mutation_probability_step {
                    *mutation_probability -= *mutation_probability_step;
                }
                log::trace!(
                    "### dynamic_once mutation probability: {}",
                    mutation_probability
                );

                let bool_sampler = Bernoulli::new(*mutation_probability as f64).unwrap();
                for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
                    if bool_sampler.sample(rng) {
                        genotype.mutate_chromosome_random(chromosome, rng);
                    }
                }
            }
            Mutate::DynamicRounds {
                mutation_probability,
                number_of_rounds,
                target_uniformity,
            } => {
                if population.fitness_score_uniformity() > *target_uniformity {
                    *number_of_rounds += 1
                } else if *number_of_rounds > 0 {
                    *number_of_rounds -= 1
                }
                log::trace!(
                    "### dynamic_rounds mutation probability: {}, rounds: {}",
                    *mutation_probability,
                    *number_of_rounds
                );

                let bool_sampler = Bernoulli::new(*mutation_probability as f64).unwrap();
                for chromosome in population.chromosomes.iter_mut().filter(|c| c.age == 0) {
                    for _ in 0..*number_of_rounds {
                        if bool_sampler.sample(rng) {
                            genotype.mutate_chromosome_random(chromosome, rng);
                        }
                    }
                }
            }
        }
    }
    pub fn report(&self) -> String {
        match self {
            Mutate::Once {
                mutation_probability,
            } => format!("once: {:2.2}", mutation_probability),
            Mutate::Twice {
                mutation_probability,
            } => format!("twice: {:2.2}", mutation_probability),
            Mutate::DynamicOnce {
                mutation_probability,
                ..
            } => format!("prob: {:2.2}", mutation_probability),
            Mutate::DynamicRounds {
                number_of_rounds, ..
            } => format!("rounds: {}", number_of_rounds),
        }
    }
}
