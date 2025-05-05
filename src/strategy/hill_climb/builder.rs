use super::{HillClimb, HillClimbVariant};
pub use crate::errors::TryFromStrategyBuilderError as TryFromBuilderError;
use crate::fitness::{Fitness, FitnessCache, FitnessOrdering, FitnessValue};
use crate::genotype::HillClimbGenotype;
use crate::strategy::Strategy;
pub use crate::strategy::{StrategyReporter, StrategyReporterNoop, StrategyState};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::sync::mpsc::channel;

/// The builder for an HillClimb struct.
#[derive(Clone, Debug)]
pub struct Builder<
    G: HillClimbGenotype,
    F: Fitness<Genotype = G>,
    SR: StrategyReporter<Genotype = G>,
> {
    pub genotype: Option<G>,
    pub variant: Option<HillClimbVariant>,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
    pub fitness_cache: Option<FitnessCache>,
    pub par_fitness: bool,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub replace_on_equal_fitness: bool,
    pub reporter: SR,
    pub rng_seed: Option<u64>,
}

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>> Default
    for Builder<G, F, StrategyReporterNoop<G>>
{
    fn default() -> Self {
        Self {
            genotype: None,
            variant: None,
            fitness: None,
            fitness_ordering: FitnessOrdering::Maximize,
            fitness_cache: None,
            par_fitness: false,
            max_stale_generations: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            replace_on_equal_fitness: true,
            reporter: StrategyReporterNoop::new(),
            rng_seed: None,
        }
    }
}
impl<G: HillClimbGenotype, F: Fitness<Genotype = G>> Builder<G, F, StrategyReporterNoop<G>> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    Builder<G, F, SR>
{
    pub fn build(self) -> Result<HillClimb<G, F, SR>, TryFromBuilderError> {
        self.try_into()
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
    /// Only works when genes_hash is stored on chromosome, as this is the cache key.
    /// Only useful for long stale runs.
    /// Silently ignore cache_size of zero, to support superset builder which delays specialization
    pub fn with_fitness_cache(mut self, fitness_cache_size: usize) -> Self {
        match FitnessCache::try_new(fitness_cache_size) {
            Ok(cache) => self.fitness_cache = Some(cache),
            Err(_error) => (),
        }
        self
    }
    pub fn with_par_fitness(mut self, par_fitness: bool) -> Self {
        self.par_fitness = par_fitness;
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
    pub fn with_valid_fitness_score(mut self, valid_fitness_score: FitnessValue) -> Self {
        self.valid_fitness_score = Some(valid_fitness_score);
        self
    }
    pub fn with_valid_fitness_score_option(
        mut self,
        valid_fitness_score_option: Option<FitnessValue>,
    ) -> Self {
        self.valid_fitness_score = valid_fitness_score_option;
        self
    }
    pub fn with_replace_on_equal_fitness(mut self, replace_on_equal_fitness: bool) -> Self {
        self.replace_on_equal_fitness = replace_on_equal_fitness;
        self
    }
    pub fn with_reporter<SR2: StrategyReporter<Genotype = G>>(
        self,
        reporter: SR2,
    ) -> Builder<G, F, SR2> {
        Builder {
            genotype: self.genotype,
            variant: self.variant,
            fitness: self.fitness,
            fitness_ordering: self.fitness_ordering,
            fitness_cache: self.fitness_cache,
            par_fitness: self.par_fitness,
            max_stale_generations: self.max_stale_generations,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            reporter,
            rng_seed: self.rng_seed,
        }
    }
    pub fn with_rng_seed_from_u64(mut self, rng_seed: u64) -> Self {
        self.rng_seed = Some(rng_seed);
        self
    }
    pub fn with_rng_seed_from_u64_option(mut self, rng_seed_option: Option<u64>) -> Self {
        self.rng_seed = rng_seed_option;
        self
    }
}

#[allow(clippy::type_complexity)]
impl<G: HillClimbGenotype, F: Fitness<Genotype = G>, SR: StrategyReporter<Genotype = G>>
    Builder<G, F, SR>
{
    pub fn rng(&self) -> SmallRng {
        if let Some(seed) = self.rng_seed {
            SmallRng::seed_from_u64(seed)
        } else {
            // SmallRng::from_entropy()
            SmallRng::from_rng(rand::thread_rng()).unwrap()
        }
    }
    pub fn call(self) -> Result<HillClimb<G, F, SR>, TryFromBuilderError> {
        let mut hill_climb: HillClimb<G, F, SR> = self.try_into()?;
        hill_climb.call();
        Ok(hill_climb)
    }

    pub fn call_repeatedly(
        self,
        max_repeats: usize,
    ) -> Result<(HillClimb<G, F, SR>, Vec<HillClimb<G, F, SR>>), TryFromBuilderError> {
        let mut runs: Vec<HillClimb<G, F, SR>> = vec![];
        (0..max_repeats)
            .filter_map(|iteration| {
                let mut contending_run: HillClimb<G, F, SR> = self.clone().try_into().ok()?;
                contending_run.state.current_iteration = iteration;
                Some(contending_run)
            })
            .map(|mut contending_run| {
                contending_run.call();
                let stop = contending_run.is_finished_by_target_fitness_score();
                runs.push(contending_run);
                stop
            })
            .any(|x| x);

        let best_run = self.extract_best_run(&mut runs);
        Ok((best_run, runs))
    }

    pub fn call_par_repeatedly(
        self,
        max_repeats: usize,
    ) -> Result<(HillClimb<G, F, SR>, Vec<HillClimb<G, F, SR>>), TryFromBuilderError> {
        let _valid_builder: HillClimb<G, F, SR> = self.clone().try_into()?;
        let mut runs: Vec<HillClimb<G, F, SR>> = vec![];
        rayon::scope(|s| {
            let builder = &self;
            let (sender, receiver) = channel();

            s.spawn(move |_| {
                (0..max_repeats)
                    .filter_map(|iteration| {
                        let mut contending_run: HillClimb<G, F, SR> =
                            builder.clone().try_into().ok()?;
                        contending_run.state.current_iteration = iteration;
                        Some(contending_run)
                    })
                    .par_bridge()
                    .map_with(sender, |sender, mut contending_run| {
                        contending_run.call();
                        let stop = contending_run.is_finished_by_target_fitness_score();
                        sender.send(contending_run).unwrap();
                        stop
                    })
                    .any(|x| x);
            });

            receiver.iter().for_each(|contending_run| {
                runs.push(contending_run);
            });
        });
        let best_run = self.extract_best_run(&mut runs);
        Ok((best_run, runs))
    }

    pub fn extract_best_run(&self, runs: &mut Vec<HillClimb<G, F, SR>>) -> HillClimb<G, F, SR> {
        let mut best_index = 0;
        let mut best_fitness_score: Option<FitnessValue> = None;
        runs.iter().enumerate().for_each(|(index, contending_run)| {
            let contending_fitness_score = contending_run.best_fitness_score();
            match (best_fitness_score, contending_fitness_score) {
                (None, None) => {}
                (Some(_), None) => {}
                (None, Some(_)) => {
                    best_index = index;
                    best_fitness_score = contending_fitness_score;
                }
                (Some(current_fitness_value), Some(contending_fitness_value)) => {
                    match self.fitness_ordering {
                        FitnessOrdering::Maximize => {
                            if contending_fitness_value >= current_fitness_value {
                                best_index = index;
                                best_fitness_score = contending_fitness_score;
                            }
                        }
                        FitnessOrdering::Minimize => {
                            if contending_fitness_value <= current_fitness_value {
                                best_index = index;
                                best_fitness_score = contending_fitness_score;
                            }
                        }
                    }
                }
            }
        });
        runs.remove(best_index)
    }
}
