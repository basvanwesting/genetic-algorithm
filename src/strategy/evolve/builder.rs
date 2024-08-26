use super::{Evolve, EvolveReporter, EvolveReporterNoop};
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::extension::{Extension, ExtensionNoop};
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::strategy::Strategy;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::sync::mpsc::channel;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for an Evolve struct.
#[derive(Clone, Debug)]
pub struct Builder<
    G: Genotype,
    M: Mutate,
    F: Fitness<Allele = G::Allele>,
    S: Crossover,
    C: Compete,
    E: Extension,
    SR: EvolveReporter<Allele = G::Allele>,
> {
    pub genotype: Option<G>,
    pub target_population_size: usize,
    pub max_stale_generations: Option<usize>,
    pub max_chromosome_age: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
    pub mutate: Option<M>,
    pub fitness: Option<F>,
    pub crossover: Option<S>,
    pub compete: Option<C>,
    pub extension: E,
    pub reporter: SR,
    pub rng_seed: Option<u64>,
}

impl<G: Genotype, M: Mutate, F: Fitness<Allele = G::Allele>, S: Crossover, C: Compete> Default
    for Builder<G, M, F, S, C, ExtensionNoop, EvolveReporterNoop<G::Allele>>
{
    fn default() -> Self {
        Self {
            genotype: None,
            target_population_size: 0,
            max_stale_generations: None,
            max_chromosome_age: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            fitness_ordering: FitnessOrdering::Maximize,
            par_fitness: false,
            replace_on_equal_fitness: false,
            mutate: None,
            fitness: None,
            crossover: None,
            compete: None,
            extension: ExtensionNoop::new(),
            reporter: EvolveReporterNoop::new(),
            rng_seed: None,
        }
    }
}
impl<G: Genotype, M: Mutate, F: Fitness<Allele = G::Allele>, S: Crossover, C: Compete>
    Builder<G, M, F, S, C, ExtensionNoop, EvolveReporterNoop<G::Allele>>
{
    pub fn new() -> Self {
        Self::default()
    }
}

#[allow(clippy::type_complexity)]
impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Allele = G::Allele>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Allele = G::Allele>,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn build(self) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        self.try_into()
    }

    pub fn with_genotype(mut self, genotype: G) -> Self {
        self.genotype = Some(genotype);
        self
    }
    pub fn with_target_population_size(mut self, target_population_size: usize) -> Self {
        self.target_population_size = target_population_size;
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
    pub fn with_max_chromosome_age(mut self, max_chromosome_age: usize) -> Self {
        self.max_chromosome_age = Some(max_chromosome_age);
        self
    }
    pub fn with_max_chromosome_age_option(
        mut self,
        max_chromosome_age_option: Option<usize>,
    ) -> Self {
        self.max_chromosome_age = max_chromosome_age_option;
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
    pub fn with_fitness_ordering(mut self, fitness_ordering: FitnessOrdering) -> Self {
        self.fitness_ordering = fitness_ordering;
        self
    }
    pub fn with_par_fitness(mut self, par_fitness: bool) -> Self {
        self.par_fitness = par_fitness;
        self
    }
    pub fn with_replace_on_equal_fitness(mut self, replace_on_equal_fitness: bool) -> Self {
        self.replace_on_equal_fitness = replace_on_equal_fitness;
        self
    }
    pub fn with_mutate(mut self, mutate: M) -> Self {
        self.mutate = Some(mutate);
        self
    }
    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }
    pub fn with_crossover(mut self, crossover: S) -> Self {
        self.crossover = Some(crossover);
        self
    }
    pub fn with_compete(mut self, compete: C) -> Self {
        self.compete = Some(compete);
        self
    }
    pub fn with_extension<E2: Extension>(self, extension: E2) -> Builder<G, M, F, S, C, E2, SR> {
        Builder {
            genotype: self.genotype,
            target_population_size: self.target_population_size,
            max_stale_generations: self.max_stale_generations,
            max_chromosome_age: self.max_chromosome_age,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            mutate: self.mutate,
            fitness: self.fitness,
            crossover: self.crossover,
            compete: self.compete,
            extension,
            reporter: self.reporter,
            rng_seed: self.rng_seed,
        }
    }
    pub fn with_reporter<SR2: EvolveReporter<Allele = G::Allele>>(
        self,
        reporter: SR2,
    ) -> Builder<G, M, F, S, C, E, SR2> {
        Builder {
            genotype: self.genotype,
            target_population_size: self.target_population_size,
            max_stale_generations: self.max_stale_generations,
            max_chromosome_age: self.max_chromosome_age,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            mutate: self.mutate,
            fitness: self.fitness,
            crossover: self.crossover,
            compete: self.compete,
            extension: self.extension,
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
impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Allele = G::Allele>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Allele = G::Allele>,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn rng(&self) -> SmallRng {
        if let Some(seed) = self.rng_seed {
            SmallRng::seed_from_u64(seed)
        } else {
            // SmallRng::from_entropy()
            SmallRng::from_rng(rand::thread_rng()).unwrap()
        }
    }
    pub fn call(self) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        let mut evolve: Evolve<G, M, F, S, C, E, SR> = self.try_into()?;
        evolve.call();
        Ok(evolve)
    }
    pub fn call_repeatedly(
        self,
        max_repeats: usize,
    ) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        let mut best_evolve: Option<Evolve<G, M, F, S, C, E, SR>> = None;
        for iteration in 0..max_repeats {
            let mut contending_run: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into()?;
            contending_run.state.current_iteration = iteration;
            contending_run.call();
            if contending_run.is_finished_by_target_fitness_score() {
                best_evolve = Some(contending_run);
                break;
            }
            if let Some(best_run) = best_evolve.as_ref() {
                match (
                    best_run.best_fitness_score(),
                    contending_run.best_fitness_score(),
                ) {
                    (None, None) => {}
                    (Some(_), None) => {}
                    (None, Some(_)) => {
                        best_evolve = Some(contending_run);
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match contending_run.config.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score >= current_fitness_score {
                                    best_evolve = Some(contending_run);
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score <= current_fitness_score {
                                    best_evolve = Some(contending_run);
                                }
                            }
                        }
                    }
                }
            } else {
                best_evolve = Some(contending_run);
            }
        }
        Ok(best_evolve.unwrap())
    }

    pub fn call_par_repeatedly(
        self,
        max_repeats: usize,
    ) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        let _valid_builder: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into()?;
        let mut best_evolve: Option<Evolve<G, M, F, S, C, E, SR>> = None;
        rayon::scope(|s| {
            let builder = &self;
            let (sender, receiver) = channel();

            s.spawn(move |_| {
                (0..max_repeats)
                    .filter_map(|iteration| {
                        let mut contending_run: Evolve<G, M, F, S, C, E, SR> =
                            builder.clone().try_into().ok()?;
                        contending_run.state.current_iteration = iteration;
                        Some(contending_run)
                    })
                    .par_bridge()
                    .map_with(sender, |sender, mut contending_run| {
                        contending_run.call();
                        let finished_by_target_fitness_score =
                            contending_run.is_finished_by_target_fitness_score();
                        sender.send(contending_run).unwrap();
                        finished_by_target_fitness_score
                    })
                    .any(|x| x);
            });

            receiver.iter().for_each(|contending_run| {
                if let Some(best_run) = best_evolve.as_ref() {
                    match (
                        best_run.best_fitness_score(),
                        contending_run.best_fitness_score(),
                    ) {
                        (None, None) => {}
                        (Some(_), None) => {}
                        (None, Some(_)) => {
                            best_evolve = Some(contending_run);
                        }
                        (Some(current_fitness_score), Some(contending_fitness_score)) => {
                            match contending_run.config.fitness_ordering {
                                FitnessOrdering::Maximize => {
                                    if contending_fitness_score >= current_fitness_score {
                                        best_evolve = Some(contending_run);
                                    }
                                }
                                FitnessOrdering::Minimize => {
                                    if contending_fitness_score <= current_fitness_score {
                                        best_evolve = Some(contending_run);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    best_evolve = Some(contending_run);
                }
            });
        });
        Ok(best_evolve.unwrap())
    }

    pub fn call_speciated(
        self,
        number_of_species: usize,
    ) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        let _valid_builder: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into()?;
        let mut species_runs: Vec<Evolve<G, M, F, S, C, E, SR>> = vec![];
        let finished_by_target_fitness_score = (0..number_of_species)
            .filter_map(|iteration| {
                let mut species_run: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into().ok()?;
                species_run.state.current_iteration = iteration;
                Some(species_run)
            })
            .map(|mut species_run| {
                species_run.call();
                let finished_by_target_fitness_score =
                    species_run.is_finished_by_target_fitness_score();
                species_runs.push(species_run);
                finished_by_target_fitness_score
            })
            .any(|x| x);

        let final_run = if finished_by_target_fitness_score {
            species_runs
                .into_iter()
                .find(|species_run| species_run.is_finished_by_target_fitness_score())
                .unwrap()
        } else {
            let seed_genes_list = species_runs
                .iter()
                .filter_map(|species_run| species_run.best_chromosome())
                .map(|best_chromosome| best_chromosome.genes.clone())
                .collect();
            let mut final_genotype = self.genotype.clone().unwrap();
            final_genotype.set_seed_genes_list(seed_genes_list);
            let mut final_run: Evolve<G, M, F, S, C, E, SR> =
                self.clone().with_genotype(final_genotype).try_into()?;

            final_run.call();
            final_run
        };
        Ok(final_run)
    }

    pub fn call_par_speciated(
        self,
        number_of_species: usize,
    ) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        let _valid_builder: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into()?;
        let mut species_runs: Vec<Evolve<G, M, F, S, C, E, SR>> = vec![];
        rayon::scope(|s| {
            let builder = &self;
            let (sender, receiver) = channel();

            s.spawn(move |_| {
                (0..number_of_species)
                    .filter_map(|iteration| {
                        let mut species_run: Evolve<G, M, F, S, C, E, SR> =
                            builder.clone().try_into().ok()?;
                        species_run.state.current_iteration = iteration;
                        Some(species_run)
                    })
                    .par_bridge()
                    .map_with(sender, |sender, mut species_run| {
                        species_run.call();
                        let finished_by_target_fitness_score =
                            species_run.is_finished_by_target_fitness_score();
                        sender.send(species_run).unwrap();
                        finished_by_target_fitness_score
                    })
                    .any(|x| x);
            });

            receiver.iter().for_each(|species_run| {
                species_runs.push(species_run);
            });
        });

        let finished_by_target_fitness_score = species_runs
            .iter()
            .any(|species_run| species_run.is_finished_by_target_fitness_score());

        let final_run = if finished_by_target_fitness_score {
            species_runs
                .into_iter()
                .find(|species_run| species_run.is_finished_by_target_fitness_score())
                .unwrap()
        } else {
            let seed_genes_list = species_runs
                .iter()
                .filter_map(|species_run| species_run.best_chromosome())
                .map(|best_chromosome| best_chromosome.genes.clone())
                .collect();
            let mut final_genotype = self.genotype.clone().unwrap();
            final_genotype.set_seed_genes_list(seed_genes_list);
            let mut final_run: Evolve<G, M, F, S, C, E, SR> =
                self.clone().with_genotype(final_genotype).try_into()?;

            final_run.call();
            final_run
        };
        Ok(final_run)
    }
}
