use super::{HillClimb, HillClimbReporter, HillClimbReporterNoop, HillClimbVariant};
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::IncrementalGenotype;
use crate::strategy::Strategy;
use rand::Rng;
use rayon::prelude::*;
use std::sync::mpsc::channel;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for an HillClimb struct.
#[derive(Clone, Debug)]
pub struct Builder<
    G: IncrementalGenotype,
    F: Fitness<Allele = G::Allele>,
    SR: HillClimbReporter<Allele = G::Allele>,
> {
    pub genotype: Option<G>,
    pub variant: Option<HillClimbVariant>,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub max_stale_generations: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub replace_on_equal_fitness: bool,
    pub reporter: SR,
}

impl<G: IncrementalGenotype, F: Fitness<Allele = G::Allele>> Default
    for Builder<G, F, HillClimbReporterNoop<G::Allele>>
{
    fn default() -> Self {
        Self {
            genotype: None,
            variant: None,
            fitness: None,
            fitness_ordering: FitnessOrdering::Maximize,
            par_fitness: false,
            max_stale_generations: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            replace_on_equal_fitness: true,
            reporter: HillClimbReporterNoop::new(),
        }
    }
}
impl<G: IncrementalGenotype, F: Fitness<Allele = G::Allele>>
    Builder<G, F, HillClimbReporterNoop<G::Allele>>
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<
        G: IncrementalGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: HillClimbReporter<Allele = G::Allele>,
    > Builder<G, F, SR>
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
    pub fn with_reporter<SR2: HillClimbReporter<Allele = G::Allele>>(
        self,
        reporter: SR2,
    ) -> Builder<G, F, SR2> {
        Builder {
            genotype: self.genotype,
            variant: self.variant,
            fitness: self.fitness,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            max_stale_generations: self.max_stale_generations,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            reporter,
        }
    }
}

impl<
        G: IncrementalGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: HillClimbReporter<Allele = G::Allele>,
    > Builder<G, F, SR>
{
    pub fn call<R: Rng>(self, rng: &mut R) -> Result<HillClimb<G, F, SR>, TryFromBuilderError> {
        let mut hill_climb: HillClimb<G, F, SR> = self.try_into()?;
        hill_climb.call(rng);
        Ok(hill_climb)
    }

    pub fn call_repeatedly<R: Rng>(
        self,
        max_repeats: usize,
        rng: &mut R,
    ) -> Result<HillClimb<G, F, SR>, TryFromBuilderError> {
        let mut best_hill_climb: Option<HillClimb<G, F, SR>> = None;
        for iteration in 0..max_repeats {
            let mut contending_run: HillClimb<G, F, SR> = self.clone().try_into()?;
            contending_run.state.current_iteration = iteration;
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
                        match contending_run.config.fitness_ordering {
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

    pub fn call_par_repeatedly<R: Rng>(
        self,
        max_repeats: usize,
        _rng: &mut R,
    ) -> Result<HillClimb<G, F, SR>, TryFromBuilderError> {
        let _valid_builder: HillClimb<G, F, SR> = self.clone().try_into()?;
        let mut best_hill_climb: Option<HillClimb<G, F, SR>> = None;
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
                    .map_init(
                        || (sender.clone(), rand::thread_rng()),
                        |(sender, rng), mut contending_run| {
                            contending_run.call(rng);
                            let stop = contending_run.is_finished_by_target_fitness_score();
                            sender.send(contending_run).unwrap();
                            stop
                        },
                    )
                    .any(|x| x);
            });

            receiver.iter().for_each(|contending_run| {
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
                            match contending_run.config.fitness_ordering {
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
            });
        });
        Ok(best_hill_climb.unwrap())
    }
}
