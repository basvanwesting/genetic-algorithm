use super::{Evolve, EvolveReporter};
use crate::chromosome::Chromosome;
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::extension::Extension;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::mutate::Mutate;
use crate::strategy::Strategy;
use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for an Evolve struct.
#[derive(Clone, Debug)]
pub struct Builder<
    G: Genotype,
    M: Mutate,
    F: Fitness<Genotype = G>,
    S: Crossover,
    C: Compete,
    E: Extension,
    SR: EvolveReporter<Genotype = G>,
> {
    pub genotype: Option<G>,
    pub target_population_size: usize,
    pub max_stale_generations: Option<usize>,
    pub max_chromosome_age: Option<usize>,
    pub target_fitness_score: Option<FitnessValue>,
    pub valid_fitness_score: Option<FitnessValue>,
    pub fitness_ordering: FitnessOrdering,
    pub multithreading: bool,
    pub mutate: Option<M>,
    pub fitness: Option<F>,
    pub crossover: Option<S>,
    pub compete: Option<C>,
    pub extension: Option<E>,
    pub reporter: Option<SR>,
}

impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Genotype = G>,
    > Default for Builder<G, M, F, S, C, E, SR>
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
            multithreading: false,
            mutate: None,
            fitness: None,
            crossover: None,
            compete: None,
            extension: None,
            reporter: None,
        }
    }
}

#[allow(clippy::type_complexity)]
impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Genotype = G>,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn new() -> Self {
        Self::default()
    }

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
    pub fn with_multithreading(mut self, multithreading: bool) -> Self {
        self.multithreading = multithreading;
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
    pub fn with_extension(mut self, extension: E) -> Self {
        self.extension = Some(extension);
        self
    }
    pub fn with_reporter(mut self, reporter: SR) -> Self {
        self.reporter = Some(reporter);
        self
    }
}

#[allow(clippy::type_complexity)]
impl<
        G: Genotype,
        M: Mutate,
        F: Fitness<Genotype = G>,
        S: Crossover,
        C: Compete,
        E: Extension,
        SR: EvolveReporter<Genotype = G>,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn call<R: Rng>(
        self,
        rng: &mut R,
    ) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        let mut evolve: Evolve<G, M, F, S, C, E, SR> = self.try_into()?;
        evolve.call(rng);
        Ok(evolve)
    }
    pub fn call_repeatedly<R: Rng>(
        self,
        max_repeats: usize,
        rng: &mut R,
    ) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        let mut best_evolve: Option<Evolve<G, M, F, S, C, E, SR>> = None;
        for iteration in 0..max_repeats {
            log::info!("### repeated round: {}", iteration);
            let mut contending_run: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into()?;
            contending_run.state.current_iteration = iteration;
            contending_run.call(rng);
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
    pub fn call_speciated<R: Rng>(
        self,
        number_of_species: usize,
        rng: &mut R,
    ) -> Result<Evolve<G, M, F, S, C, E, SR>, TryFromBuilderError> {
        let best_chromosomes: Vec<Chromosome<G>> = (0..number_of_species)
            .filter_map(|iteration| {
                log::info!("### speciated round: {}", iteration);
                let mut species_run: Evolve<G, M, F, S, C, E, SR> = self.clone().try_into().ok()?;
                species_run.state.current_iteration = iteration;
                species_run.call(rng);
                species_run.best_chromosome()
            })
            .collect();

        log::info!("### speciated final run");
        let seed_genes_list = best_chromosomes
            .iter()
            .map(|best_chromosome| best_chromosome.genes.clone())
            .collect();
        log::debug!("### seed_genes_list: {:?}", seed_genes_list);
        let mut final_genotype = self.genotype.clone().unwrap();
        final_genotype.set_seed_genes_list(seed_genes_list);
        let mut final_run: Evolve<G, M, F, S, C, E, SR> =
            self.clone().with_genotype(final_genotype).try_into()?;

        final_run.call(rng);
        Ok(final_run)
    }
}
