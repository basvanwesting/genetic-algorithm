use super::{HillClimb, HillClimbVariant, RandomChromosomeProbability};
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::strategy::Strategy;
use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for an HillClimb struct.
#[derive(Clone, Debug)]
pub struct Builder<G: Genotype, F: Fitness<Genotype = G>> {
    pub genotype: Option<G>,
    pub variant: Option<HillClimbVariant>,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub random_chromosome_probability: Option<RandomChromosomeProbability>,
}

impl<G: Genotype, F: Fitness<Genotype = G>> Builder<G, F> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Result<HillClimb<G, F>, TryFromBuilderError> {
        self.try_into()
    }
    pub fn call<R: Rng>(self, rng: &mut R) -> Result<HillClimb<G, F>, TryFromBuilderError> {
        let mut hill_climb: HillClimb<G, F> = self.try_into()?;
        hill_climb.call(rng);
        Ok(hill_climb)
    }
    pub fn call_repeatedly<R: Rng>(
        self,
        max_repeats: usize,
        rng: &mut R,
    ) -> Result<HillClimb<G, F>, TryFromBuilderError> {
        let mut best_hill_climb: Option<HillClimb<G, F>> = None;
        for iteration in 0..max_repeats {
            let mut contending_run: HillClimb<G, F> = self.clone().try_into()?;
            contending_run.current_iteration = iteration;
            contending_run.call(rng);
            if contending_run.is_finished_by_target_fitness_score() {
                best_hill_climb = Some(contending_run);
                break;
            }
            if let Some(best_run) = best_hill_climb.as_ref() {
                match (
                    best_run.best_fitness_score(),
                    contending_run.best_fitness_score(),
                ) {
                    (None, None) => {}
                    (Some(_), None) => {}
                    (None, Some(_)) => {
                        best_hill_climb = Some(contending_run);
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match contending_run.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score >= current_fitness_score {
                                    best_hill_climb = Some(contending_run);
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score <= current_fitness_score {
                                    best_hill_climb = Some(contending_run);
                                }
                            }
                        }
                    }
                }
            } else {
                best_hill_climb = Some(contending_run);
            }
        }
        Ok(best_hill_climb.unwrap())
    }

    pub fn with_genotype(mut self, genotype: G) -> Self {
        self.genotype = Some(genotype);
        self
    }
    pub fn with_variant(mut self, variant: HillClimbVariant) -> Self {
        self.variant = Some(variant);
        self
    }
    pub fn with_fitness_ordering(mut self, fitness_ordering: FitnessOrdering) -> Self {
        self.fitness_ordering = fitness_ordering;
        self
    }
    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }
    pub fn with_max_stale_generations(mut self, max_stale_generations: usize) -> Self {
        self.max_stale_generations = Some(max_stale_generations);
        self
    }
    pub fn with_max_stale_generations_option(
        mut self,
        max_stale_generations_option: Option<usize>,
    ) -> Self {
        self.max_stale_generations = max_stale_generations_option;
        self
    }
    pub fn with_target_fitness_score(mut self, target_fitness_score: FitnessValue) -> Self {
        self.target_fitness_score = Some(target_fitness_score);
        self
    }
    pub fn with_target_fitness_score_option(
        mut self,
        target_fitness_score_option: Option<FitnessValue>,
    ) -> Self {
        self.target_fitness_score = target_fitness_score_option;
        self
    }
    pub fn with_random_chromosome_probability(
        mut self,
        random_chromosome_probability: RandomChromosomeProbability,
    ) -> Self {
        self.random_chromosome_probability = Some(random_chromosome_probability);
        self
    }
}

impl<G: Genotype, F: Fitness<Genotype = G>> Default for Builder<G, F> {
    fn default() -> Self {
        Self {
            genotype: None,
            variant: None,
            fitness: None,
            fitness_ordering: FitnessOrdering::Maximize,
            max_stale_generations: None,
            target_fitness_score: None,
            random_chromosome_probability: None,
        }
    }
}
