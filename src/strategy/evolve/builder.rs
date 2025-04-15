use super::Evolve;
use crate::crossover::Crossover;
pub use crate::errors::TryFromStrategyBuilderError as TryFromBuilderError;
use crate::extension::{Extension, ExtensionNoop};
use crate::fitness::{Fitness, FitnessCache, FitnessOrdering, FitnessValue};
use crate::genotype::EvolveGenotype;
use crate::mutate::Mutate;
use crate::select::Select;
use crate::strategy::{Strategy, StrategyReporter, StrategyReporterNoop};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::sync::mpsc::channel;

/// The builder for an Evolve struct.
#[derive(Clone, Debug)]
pub struct Builder<
    G: EvolveGenotype,
    M: Mutate,
    F: Fitness<Genotype = G>,
    S: Crossover,
    C: Select,
    E: Extension,
    SR: StrategyReporter<Genotype = G>,
> {
    pub genotype: Option<G>,
    pub target_population_size: usize,
    pub max_stale_generations: Option<usize>,
    pub max_chromosome_age: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub fitness_ordering: FitnessOrdering,
    pub fitness_cache: Option<FitnessCache>,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
    pub mutate: Option<M>,
    pub fitness: Option<F>,
    pub crossover: Option<S>,
    pub select: Option<C>,
    pub extension: E,
    pub reporter: SR,
    pub rng_seed: Option<u64>,
}

impl<G: EvolveGenotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Select> Default
    for Builder<G, M, F, S, C, ExtensionNoop, StrategyReporterNoop<G>>
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
            fitness_cache: None,
            par_fitness: false,
            replace_on_equal_fitness: false,
            mutate: None,
            fitness: None,
            crossover: None,
            select: None,
            extension: ExtensionNoop::new(),
            reporter: StrategyReporterNoop::new(),
            rng_seed: None,
        }
    }
}
impl<G: EvolveGenotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Select>
    Builder<G, M, F, S, C, ExtensionNoop, StrategyReporterNoop<G>>
{
    pub fn new() -> Self {
        Self::default()
    }
}

#[allow(clippy::type_complexity)]
impl<
        G: EvolveGenotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Select,
        E: Extension,
        SR: StrategyReporter<Genotype = G>,
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
    /// Only works when genes_hash is stored on chromosome, as this is the cache key.
    /// Only useful for long stale runs, but better to increase population diversity.
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
    pub fn with_select(mut self, select: C) -> Self {
        self.select = Some(select);
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
            fitness_cache: self.fitness_cache,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            mutate: self.mutate,
            fitness: self.fitness,
            crossover: self.crossover,
            select: self.select,
            extension,
            reporter: self.reporter,
            rng_seed: self.rng_seed,
        }
    }
    pub fn with_reporter<SR2: StrategyReporter<Genotype = G>>(
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
            fitness_cache: self.fitness_cache,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            mutate: self.mutate,
            fitness: self.fitness,
            crossover: self.crossover,
            select: self.select,
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
        G: EvolveGenotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Select,
        E: Extension,
        SR: StrategyReporter<Genotype = G>,
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
    ) -> Result<
        (
            Evolve<G, M, F, S, C, E, SR>,
            Vec<Evolve<G, M, F, S, C, E, SR>>,
        ),
        TryFromBuilderError,
    > {
        let mut runs: Vec<Evolve<G, M, F, S, C, E, SR>> = vec![];
        (0..max_repeats)
            .filter_map(|iteration| {
                let mut contending_run: Evolve<G, M, F, S, C, E, SR> =
                    self.clone().try_into().ok()?;
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
    ) -> Result<
        (
            Evolve<G, M, F, S, C, E, SR>,
            Vec<Evolve<G, M, F, S, C, E, SR>>,
        ),
        TryFromBuilderError,
    > {
        let _valid_builder: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into()?;
        let mut runs: Vec<Evolve<G, M, F, S, C, E, SR>> = vec![];
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

    pub fn call_speciated(
        self,
        number_of_species: usize,
    ) -> Result<
        (
            Evolve<G, M, F, S, C, E, SR>,
            Vec<Evolve<G, M, F, S, C, E, SR>>,
        ),
        TryFromBuilderError,
    > {
        let _valid_builder: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into()?;
        let mut species_runs: Vec<Evolve<G, M, F, S, C, E, SR>> = vec![];
        (0..number_of_species)
            .filter_map(|iteration| {
                let mut species_run: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into().ok()?;
                species_run.state.current_iteration = iteration;
                Some(species_run)
            })
            .map(|mut species_run| {
                species_run.call();
                let stop = species_run.is_finished_by_target_fitness_score();
                species_runs.push(species_run);
                stop
            })
            .any(|x| x);

        let final_run = if let Some(index_finished_by_target_fitness_score) = species_runs
            .iter()
            .position(|species_run| species_run.is_finished_by_target_fitness_score())
        {
            species_runs.remove(index_finished_by_target_fitness_score)
        } else {
            let seed_genes_list = species_runs
                .iter()
                .filter_map(|species_run| species_run.best_genes())
                .collect();
            let mut final_genotype = self.genotype.clone().unwrap();
            final_genotype.set_seed_genes_list(seed_genes_list);
            let mut final_run: Evolve<G, M, F, S, C, E, SR> =
                self.clone().with_genotype(final_genotype).try_into()?;

            final_run.call();
            final_run
        };
        Ok((final_run, species_runs))
    }

    pub fn call_par_speciated(
        self,
        number_of_species: usize,
    ) -> Result<
        (
            Evolve<G, M, F, S, C, E, SR>,
            Vec<Evolve<G, M, F, S, C, E, SR>>,
        ),
        TryFromBuilderError,
    > {
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
                        let stop = species_run.is_finished_by_target_fitness_score();
                        sender.send(species_run).unwrap();
                        stop
                    })
                    .any(|x| x);
            });

            receiver.iter().for_each(|species_run| {
                species_runs.push(species_run);
            });
        });

        let final_run = if let Some(index_finished_by_target_fitness_score) = species_runs
            .iter()
            .position(|species_run| species_run.is_finished_by_target_fitness_score())
        {
            species_runs.remove(index_finished_by_target_fitness_score)
        } else {
            let seed_genes_list = species_runs
                .iter()
                .filter_map(|species_run| species_run.best_genes())
                .collect();
            let mut final_genotype = self.genotype.clone().unwrap();
            final_genotype.set_seed_genes_list(seed_genes_list);
            let mut final_run: Evolve<G, M, F, S, C, E, SR> =
                self.clone().with_genotype(final_genotype).try_into()?;

            final_run.call();
            final_run
        };
        Ok((final_run, species_runs))
    }

    pub fn extract_best_run(
        &self,
        runs: &mut Vec<Evolve<G, M, F, S, C, E, SR>>,
    ) -> Evolve<G, M, F, S, C, E, SR> {
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
