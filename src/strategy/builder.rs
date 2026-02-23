use crate::crossover::Crossover;
pub use crate::errors::TryFromStrategyBuilderError as TryFromBuilderError;
use crate::extension::{Extension, ExtensionNoop};
use crate::fitness::{Fitness, FitnessCache, FitnessOrdering, FitnessValue};
use crate::genotype::{EvolveGenotype, HillClimbGenotype, PermutateGenotype};
use crate::mutate::Mutate;
use crate::select::Select;
use crate::strategy::evolve::EvolveBuilder;
use crate::strategy::hill_climb::HillClimbBuilder;
use crate::strategy::permutate::PermutateBuilder;
use crate::strategy::{Strategy, StrategyReporter, StrategyReporterNoop, StrategyVariant};

/// The superset builder for all strategies.
///
/// *Note: Only Genotypes which implement all strategies are eligible for the superset builder.*
/// *All standard genotypes qualify. RangeGenotype/MultiRangeGenotype support Permutation only*
/// *with MutationType::Step, StepScaled, or Discrete (runtime check via allows_permutation()).*
#[derive(Clone, Debug)]
pub struct Builder<
    G: EvolveGenotype + HillClimbGenotype + PermutateGenotype,
    M: Mutate,
    F: Fitness<Genotype = G>,
    S: Crossover,
    C: Select,
    E: Extension,
    SR: StrategyReporter<Genotype = G>,
> {
    pub genotype: Option<G>,
    pub variant: Option<StrategyVariant>,
    pub crossover: Option<S>,
    pub extension: E,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
    pub fitness_cache: Option<FitnessCache>,
    pub max_chromosome_age: Option<usize>,
    pub max_stale_generations: Option<usize>,
    pub max_generations: Option<usize>,
    pub mutate: Option<M>,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
    pub reporter: SR,
    pub rng_seed: Option<u64>,
    pub select: Option<C>,
    pub target_fitness_score: Option<FitnessValue>,
    pub target_population_size: usize,
    pub valid_fitness_score: Option<FitnessValue>,
}

impl<
        G: EvolveGenotype + HillClimbGenotype + PermutateGenotype,
        M: Mutate<Genotype = G>,
        F: Fitness<Genotype = G>,
        S: Crossover<Genotype = G>,
        C: Select,
    > Default for Builder<G, M, F, S, C, ExtensionNoop<G>, StrategyReporterNoop<G>>
{
    fn default() -> Self {
        Self {
            genotype: None,
            variant: None,
            target_population_size: 0,
            max_stale_generations: None,
            max_generations: None,
            max_chromosome_age: None,
            target_fitness_score: None,
            valid_fitness_score: None,
            fitness_ordering: FitnessOrdering::Maximize,
            fitness_cache: None,
            par_fitness: false,
            replace_on_equal_fitness: true,
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
impl<
        G: EvolveGenotype + HillClimbGenotype + PermutateGenotype,
        M: Mutate<Genotype = G>,
        F: Fitness<Genotype = G>,
        S: Crossover<Genotype = G>,
        C: Select,
    > Builder<G, M, F, S, C, ExtensionNoop<G>, StrategyReporterNoop<G>>
{
    pub fn new() -> Self {
        Self::default()
    }
}

#[allow(clippy::type_complexity)]
impl<
        G: EvolveGenotype + HillClimbGenotype + PermutateGenotype,
        M: Mutate<Genotype = G>,
        F: Fitness<Genotype = G>,
        S: Crossover<Genotype = G>,
        C: Select<Genotype = G>,
        E: Extension<Genotype = G>,
        SR: StrategyReporter<Genotype = G>,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn with_genotype(mut self, genotype: G) -> Self {
        self.genotype = Some(genotype);
        self
    }
    pub fn with_variant(mut self, variant: StrategyVariant) -> Self {
        self.variant = Some(variant);
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
    pub fn with_max_generations(mut self, max_generations: usize) -> Self {
        self.max_generations = Some(max_generations);
        self
    }
    pub fn with_max_generations_option(mut self, max_generations_option: Option<usize>) -> Self {
        self.max_generations = max_generations_option;
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
    pub fn with_extension<E2: Extension<Genotype = G>>(
        self,
        extension: E2,
    ) -> Builder<G, M, F, S, C, E2, SR> {
        Builder {
            genotype: self.genotype,
            variant: self.variant,
            target_population_size: self.target_population_size,
            max_stale_generations: self.max_stale_generations,
            max_generations: self.max_generations,
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
            variant: self.variant,
            target_population_size: self.target_population_size,
            max_stale_generations: self.max_stale_generations,
            max_generations: self.max_generations,
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
        'a,
        G: EvolveGenotype + HillClimbGenotype + PermutateGenotype + 'a,
        M: Mutate<Genotype = G> + 'a,
        F: Fitness<Genotype = G> + 'a,
        S: Crossover<Genotype = G> + 'a,
        C: Select<Genotype = G> + 'a,
        E: Extension<Genotype = G> + 'a,
        SR: StrategyReporter<Genotype = G> + 'a,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn build(self) -> Result<Box<dyn Strategy<G> + 'a>, TryFromBuilderError> {
        match self.variant {
            Some(StrategyVariant::Permutate(_)) => {
                Ok(Box::new(self.to_permutate_builder().build()?))
            }
            Some(StrategyVariant::Evolve(_)) => Ok(Box::new(self.to_evolve_builder().build()?)),
            Some(StrategyVariant::HillClimb(hill_climb_variant)) => Ok(Box::new(
                self.to_hill_climb_builder()
                    .with_variant(hill_climb_variant)
                    .build()?,
            )),
            None => Err(TryFromBuilderError("StrategyVariant is required")),
        }
    }
    pub fn to_permutate_builder(self) -> PermutateBuilder<G, F, SR> {
        PermutateBuilder {
            genotype: self.genotype,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            fitness: self.fitness,
            reporter: self.reporter,
        }
    }
    pub fn to_evolve_builder(self) -> EvolveBuilder<G, M, F, S, C, E, SR> {
        EvolveBuilder {
            genotype: self.genotype,
            target_population_size: self.target_population_size,
            max_stale_generations: self.max_stale_generations,
            max_generations: self.max_generations,
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
            reporter: self.reporter,
            rng_seed: self.rng_seed,
        }
    }
    pub fn to_hill_climb_builder(self) -> HillClimbBuilder<G, F, SR> {
        HillClimbBuilder {
            genotype: self.genotype,
            variant: None,
            max_stale_generations: self.max_stale_generations,
            max_generations: self.max_generations,
            target_fitness_score: self.target_fitness_score,
            valid_fitness_score: self.valid_fitness_score,
            fitness_ordering: self.fitness_ordering,
            fitness_cache: self.fitness_cache,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            fitness: self.fitness,
            reporter: self.reporter,
            rng_seed: self.rng_seed,
        }
    }
}

#[allow(clippy::type_complexity)]
impl<
        'a,
        G: EvolveGenotype + HillClimbGenotype + PermutateGenotype + 'a,
        M: Mutate<Genotype = G> + 'a,
        F: Fitness<Genotype = G> + 'a,
        S: Crossover<Genotype = G> + 'a,
        C: Select<Genotype = G> + 'a,
        E: Extension<Genotype = G> + 'a,
        SR: StrategyReporter<Genotype = G> + 'a,
    > Builder<G, M, F, S, C, E, SR>
{
    pub fn call(self) -> Result<Box<dyn Strategy<G> + 'a>, TryFromBuilderError> {
        let mut strategy = self.build()?;
        strategy.call();
        Ok(strategy)
    }

    /// Permutate: call (once)
    /// Evolve: call_repeatedly
    /// HillClimb: call_repeatedly
    pub fn call_repeatedly(
        self,
        max_repeats: usize,
    ) -> Result<(Box<dyn Strategy<G> + 'a>, Vec<Box<dyn Strategy<G> + 'a>>), TryFromBuilderError>
    {
        match self.variant {
            Some(StrategyVariant::Permutate(_)) => {
                let run = self.to_permutate_builder().call()?;
                Ok((Box::new(run), vec![]))
            }
            Some(StrategyVariant::Evolve(_)) => {
                let (run, runs) = self.to_evolve_builder().call_repeatedly(max_repeats)?;
                Ok((
                    Box::new(run),
                    runs.into_iter().map(|r| Box::new(r) as _).collect(),
                ))
            }
            Some(StrategyVariant::HillClimb(hill_climb_variant)) => {
                let (run, runs) = self
                    .to_hill_climb_builder()
                    .with_variant(hill_climb_variant)
                    .call_repeatedly(max_repeats)?;
                Ok((
                    Box::new(run),
                    runs.into_iter().map(|r| Box::new(r) as _).collect(),
                ))
            }
            None => Err(TryFromBuilderError("StrategyVariant is required")),
        }
    }

    /// Permutate: call (force with_par_fitness)
    /// Evolve: call_par_repeatedly
    /// HillClimb: call_par_repeatedly
    pub fn call_par_repeatedly(
        self,
        max_repeats: usize,
    ) -> Result<(Box<dyn Strategy<G> + 'a>, Vec<Box<dyn Strategy<G> + 'a>>), TryFromBuilderError>
    {
        match self.variant {
            Some(StrategyVariant::Permutate(_)) => {
                let run = self.to_permutate_builder().with_par_fitness(true).call()?;
                Ok((Box::new(run), vec![]))
            }
            Some(StrategyVariant::Evolve(_)) => {
                let (run, runs) = self.to_evolve_builder().call_par_repeatedly(max_repeats)?;
                Ok((
                    Box::new(run),
                    runs.into_iter().map(|r| Box::new(r) as _).collect(),
                ))
            }
            Some(StrategyVariant::HillClimb(hill_climb_variant)) => {
                let (run, runs) = self
                    .to_hill_climb_builder()
                    .with_variant(hill_climb_variant)
                    .call_par_repeatedly(max_repeats)?;
                Ok((
                    Box::new(run),
                    runs.into_iter().map(|r| Box::new(r) as _).collect(),
                ))
            }
            None => Err(TryFromBuilderError("StrategyVariant is required")),
        }
    }

    /// Permutate: call (once)
    /// Evolve: call_speciated
    /// HillClimb: call_repeatedly
    pub fn call_speciated(
        self,
        number_of_species: usize,
    ) -> Result<(Box<dyn Strategy<G> + 'a>, Vec<Box<dyn Strategy<G> + 'a>>), TryFromBuilderError>
    {
        match self.variant {
            Some(StrategyVariant::Permutate(_)) => {
                let run = self.to_permutate_builder().call()?;
                Ok((Box::new(run), vec![]))
            }
            Some(StrategyVariant::Evolve(_)) => {
                let (run, runs) = self.to_evolve_builder().call_speciated(number_of_species)?;
                Ok((
                    Box::new(run),
                    runs.into_iter().map(|r| Box::new(r) as _).collect(),
                ))
            }
            Some(StrategyVariant::HillClimb(hill_climb_variant)) => {
                let (run, runs) = self
                    .to_hill_climb_builder()
                    .with_variant(hill_climb_variant)
                    .call_repeatedly(number_of_species)?;
                Ok((
                    Box::new(run),
                    runs.into_iter().map(|r| Box::new(r) as _).collect(),
                ))
            }
            None => Err(TryFromBuilderError("StrategyVariant is required")),
        }
    }

    /// Permutate: call (force with_par_fitness)
    /// Evolve: call_par_speciated
    /// HillClimb: call_par_repeatedly
    pub fn call_par_speciated(
        self,
        number_of_species: usize,
    ) -> Result<(Box<dyn Strategy<G> + 'a>, Vec<Box<dyn Strategy<G> + 'a>>), TryFromBuilderError>
    {
        match self.variant {
            Some(StrategyVariant::Permutate(_)) => {
                let run = self.to_permutate_builder().with_par_fitness(true).call()?;
                Ok((Box::new(run), vec![]))
            }
            Some(StrategyVariant::Evolve(_)) => {
                let (run, runs) = self
                    .to_evolve_builder()
                    .call_par_speciated(number_of_species)?;
                Ok((
                    Box::new(run),
                    runs.into_iter().map(|r| Box::new(r) as _).collect(),
                ))
            }
            Some(StrategyVariant::HillClimb(hill_climb_variant)) => {
                let (run, runs) = self
                    .to_hill_climb_builder()
                    .with_variant(hill_climb_variant)
                    .call_par_repeatedly(number_of_species)?;
                Ok((
                    Box::new(run),
                    runs.into_iter().map(|r| Box::new(r) as _).collect(),
                ))
            }
            None => Err(TryFromBuilderError("StrategyVariant is required")),
        }
    }
}
