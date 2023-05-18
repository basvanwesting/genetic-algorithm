//! A solution strategy for finding the best chromosome using evolution
mod builder;
pub mod prelude;

pub use self::builder::{
    Builder as EvolveBuilder, TryFromBuilderError as TryFromEvolveBuilderError,
};

use super::Strategy;
use crate::chromosome::Chromosome;
use crate::compete::Compete;
use crate::crossover::Crossover;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::Genotype;
use crate::mass_degeneration::MassDegeneration;
use crate::mass_extinction::MassExtinction;
use crate::mutate::Mutate;
use crate::population::Population;
use rand::Rng;
use std::cell::RefCell;
use std::fmt;
use thread_local::ThreadLocal;

/// The Evolve strategy initializes with a random population of chromosomes (unless the genotype
/// seeds specific genes to start with).
/// Then the Evolve strategy runs through generations of chromosomes in a loop:
/// * [crossover](crate::crossover) to produce new offspring with a mix of parents chromosome genes
/// * [mutate](crate::mutate) a subset of chromosomes to add some additional diversity
/// * calculate [fitness](crate::fitness) for all chromosomes
/// * [compete](crate::compete) to pair up chromosomes for crossover in next generation and drop excess chromosomes
/// * store best chromosome
/// * check ending conditions
///
/// The ending conditions are one or more of the following:
/// * target_fitness_score: when the ultimate goal in terms of fitness score is known and reached
/// * max_stale_generations: when the ultimate goal in terms of fitness score is unknown and one depends on some convergion
///   threshold, or one wants a duration limitation next to the target_fitness_score
///
/// When approacking a (local) optimum in the fitness score, the variation in the population goes
/// down dramatically. This reduces the efficiency, but also has the risk of local optimum lock-in.
/// To increase the variation in the population, one of the following mechanisms can optionally be used:
/// * [mass-extinction](crate::mass_extinction): Simulates a cambrian explosion. The controlling metric is
///   fitness score uniformity in the population (a fraction of the population which has the same
///   fitness score). When this uniformity passes the threshold, the population is randomly reduced
///   using the survival_rate (fraction of population).
/// * [mass-degeneration](crate::mass_degeneration): Simulates a cambrian explosion. The controlling metric is fitness score
///   uniformity in the population (a fraction of the population which has the same fitness score).
///   When this uniformity passes the threshold, the population is randomly mutated N number of rounds.
///
/// Can call_repeatedly from the [EvolveBuilder], when solution has tendency to get stuck in local
/// optimum
///
/// See [EvolveBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::evolve::prelude::*;
/// use genetic_algorithm::fitness::placeholders::CountTrue;
///
/// // the search space
/// let genotype = BinaryGenotype::builder() // boolean alleles
///     .with_genes_size(100)                // 100 genes per chromosome
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let mut rng = rand::thread_rng();           // a randomness provider implementing Trait rand::Rng
/// let evolve = Evolve::builder()
///     .with_genotype(genotype)
///     .with_population_size(100)              // evolve with 100 chromosomes
///     .with_target_fitness_score(0)           // ending condition if 0 times true in the best chromosome
///     .with_valid_fitness_score(10)           // block ending conditions until at most a 10 times true in the best chromosome
///     .with_max_stale_generations(1000)       // stop searching if there is no improvement in fitness score for 1000 generations
///     .with_mass_degeneration(MassDegeneration::new(0.9, 10))  // simulate cambrian explosion by mass degeneration, when reaching 90% uniformity, apply 10 rounds of random mutation
///     .with_mass_extinction(MassExtinction::new(0.9, 0.1))     // simulate cambrian explosion by mass extinction, when reaching 90% uniformity, trim to 10% of population
///     .with_fitness(CountTrue)                // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_multithreading(true)              // use all cores for calculating the fitness of the population
///     .with_crossover(CrossoverUniform(true)) // crossover all individual genes between 2 chromosomes for offspring
///     .with_mutate(MutateOnce(0.2))           // mutate a single gene with a 20% probability per chromosome
///     .with_compete(CompeteElite)             // sort the chromosomes by fitness to determine crossover order
///     .call(&mut rng)
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = evolve.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes, vec![false; 100])
/// ```
pub struct Evolve<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> {
    genotype: G,
    mutate: M,
    fitness: F,
    crossover: S,
    compete: C,

    population_size: usize,
    max_stale_generations: Option<usize>,
    target_fitness_score: Option<FitnessValue>,
    valid_fitness_score: Option<FitnessValue>,
    fitness_ordering: FitnessOrdering,
    multithreading: bool,
    mass_degeneration: Option<MassDegeneration>,
    mass_extinction: Option<MassExtinction>,

    pub current_iteration: usize,
    pub current_generation: usize,
    pub best_generation: usize,
    best_chromosome: Option<Chromosome<G>>,
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> Strategy<G>
    for Evolve<G, M, F, S, C>
{
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.current_generation = 0;
        self.best_generation = 0;
        let population = &mut self.population_factory(rng);

        let mut fitness_thread_local: Option<ThreadLocal<RefCell<F>>> = None;
        if self.multithreading {
            fitness_thread_local = Some(ThreadLocal::new());
        }

        while !self.is_finished() {
            self.current_generation += 1;

            self.try_mass_degeneration(population, rng);
            self.try_mass_extinction(population, rng);

            self.crossover.call(&self.genotype, population, rng);
            self.mutate.call(&self.genotype, population, rng);
            self.fitness
                .call_for_population(population, fitness_thread_local.as_ref());
            self.compete
                .call(population, self.fitness_ordering, self.population_size, rng);
            self.update_best_chromosome(population);
            self.report_round(population);
        }
    }
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.best_chromosome.clone()
    }
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete>
    Evolve<G, M, F, S, C>
{
    pub fn builder() -> EvolveBuilder<G, M, F, S, C> {
        EvolveBuilder::new()
    }

    fn update_best_chromosome(&mut self, population: &Population<G>) {
        match (
            self.best_chromosome.as_ref(),
            population.best_chromosome(self.fitness_ordering),
        ) {
            (None, None) => {}
            (Some(_), None) => {}
            (None, Some(contending_best_chromosome)) => {
                self.best_chromosome = Some(contending_best_chromosome.clone());
                self.best_generation = self.current_generation;
            }
            (Some(current_best_chromosome), Some(contending_best_chromosome)) => {
                match (
                    current_best_chromosome.fitness_score,
                    contending_best_chromosome.fitness_score,
                ) {
                    (None, None) => {}
                    (Some(_), None) => {}
                    (None, Some(_)) => {
                        self.best_chromosome = Some(contending_best_chromosome.clone());
                        self.best_generation = self.current_generation;
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match self.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score > current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    self.best_generation = self.current_generation;
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score < current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    self.best_generation = self.current_generation;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn try_mass_degeneration<R: Rng>(&mut self, population: &mut Population<G>, rng: &mut R) {
        if let Some(mass_degeneration) = &self.mass_degeneration {
            if population.fitness_score_uniformity() >= mass_degeneration.uniformity_threshold {
                log::debug!("### mass degeneration event");
                for _ in 0..mass_degeneration.number_of_rounds {
                    self.mutate.call(&self.genotype, population, rng);
                }
            }
        }
    }

    fn try_mass_extinction<R: Rng>(&mut self, population: &mut Population<G>, rng: &mut R) {
        if let Some(mass_extinction) = &self.mass_extinction {
            if population.size() == self.population_size {
                if population.fitness_score_uniformity() >= mass_extinction.uniformity_threshold {
                    log::debug!("### mass extinction event");
                    population.trim(mass_extinction.survival_rate, rng);
                }
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.allow_finished_by_valid_fitness_score()
            && (self.is_finished_by_max_stale_generations()
                || self.is_finished_by_target_fitness_score())
    }

    fn is_finished_by_max_stale_generations(&self) -> bool {
        if let Some(max_stale_generations) = self.max_stale_generations {
            self.current_generation - self.best_generation >= max_stale_generations
        } else {
            false
        }
    }

    fn is_finished_by_target_fitness_score(&self) -> bool {
        if let Some(target_fitness_score) = self.target_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                match self.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= target_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= target_fitness_score,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn allow_finished_by_valid_fitness_score(&self) -> bool {
        if let Some(valid_fitness_score) = self.valid_fitness_score {
            if let Some(fitness_score) = self.best_fitness_score() {
                match self.fitness_ordering {
                    FitnessOrdering::Maximize => fitness_score >= valid_fitness_score,
                    FitnessOrdering::Minimize => fitness_score <= valid_fitness_score,
                }
            } else {
                true
            }
        } else {
            true
        }
    }

    fn report_round(&self, population: &Population<G>) {
        log::debug!(
            "generation (current/best): {}/{}, fitness score (best/count/median/mean/stddev/uniformity): {:?} / {} / {:?} / {:.0} / {:.0} / {:2.2}",
            self.current_generation,
            self.best_generation,
            self.best_fitness_score(),
            population.fitness_score_count(),
            population.fitness_score_median(),
            population.fitness_score_mean(),
            population.fitness_score_stddev(),
            population.fitness_score_uniformity(),
        );
        log::trace!(
            "best - fitness score: {:?}, genes: {:?}",
            self.best_fitness_score(),
            self.best_chromosome
                .as_ref()
                .map_or(vec![], |c| c.genes.clone()),
        );
    }

    pub fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }

    pub fn population_factory<R: Rng>(&mut self, rng: &mut R) -> Population<G> {
        (0..self.population_size)
            .map(|_| self.genotype.chromosome_factory(rng))
            .collect::<Vec<_>>()
            .into()
    }
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete>
    TryFrom<EvolveBuilder<G, M, F, S, C>> for Evolve<G, M, F, S, C>
{
    type Error = TryFromEvolveBuilderError;

    fn try_from(builder: EvolveBuilder<G, M, F, S, C>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromEvolveBuilderError("Evolve requires a Genotype"))
        } else if builder.fitness.is_none() {
            Err(TryFromEvolveBuilderError("Evolve requires a Fitness"))
        } else if builder.mutate.is_none() {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a Mutate strategy",
            ))
        } else if builder.crossover.is_none() {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a Crossover strategy",
            ))
        } else if builder.compete.is_none() {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a Compete strategy",
            ))
        } else if builder
            .crossover
            .as_ref()
            .map(|o| o.require_crossover_indexes())
            .unwrap()
            && builder
                .genotype
                .as_ref()
                .map(|o| o.crossover_indexes().is_empty())
                .unwrap()
        {
            Err(TryFromEvolveBuilderError(
                "The provided Crossover strategy requires crossover_indexes, which the provided Genotype does not provide",
            ))
        } else if builder
            .crossover
            .as_ref()
            .map(|o| o.require_crossover_points())
            .unwrap()
            && builder
                .genotype
                .as_ref()
                .map(|o| o.crossover_points().is_empty())
                .unwrap()
        {
            Err(TryFromEvolveBuilderError(
                "The provided Crossover strategy requires crossover_points, which the provided Genotype does not provide",
            ))
        } else if !(builder.population_size > 0) {
            Err(TryFromEvolveBuilderError(
                "Evolve requires a population_size > 0",
            ))
        } else if builder.max_stale_generations.is_none() && builder.target_fitness_score.is_none()
        {
            Err(TryFromEvolveBuilderError(
                "Evolve requires at least a max_stale_generations or target_fitness_score ending condition",
            ))
        } else {
            Ok(Self {
                genotype: builder.genotype.unwrap(),
                mutate: builder.mutate.unwrap(),
                fitness: builder.fitness.unwrap(),
                crossover: builder.crossover.unwrap(),
                compete: builder.compete.unwrap(),

                population_size: builder.population_size,
                max_stale_generations: builder.max_stale_generations,
                target_fitness_score: builder.target_fitness_score,
                valid_fitness_score: builder.valid_fitness_score,
                fitness_ordering: builder.fitness_ordering,
                multithreading: builder.multithreading,
                mass_degeneration: builder.mass_degeneration,
                mass_extinction: builder.mass_extinction,

                current_iteration: 0,
                current_generation: 0,
                best_generation: 0,
                best_chromosome: None,
            })
        }
    }
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> fmt::Display
    for Evolve<G, M, F, S, C>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "evolve:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  mutate: {:?}", self.mutate)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;
        writeln!(f, "  crossover: {:?}", self.crossover)?;
        writeln!(f, "  compete: {:?}", self.compete)?;

        writeln!(f, "  population_size: {}", self.population_size)?;
        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  valid_fitness_score: {:?}", self.valid_fitness_score)?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  multithreading: {:?}", self.multithreading)?;
        writeln!(f, "  mass_degeneration: {:?}", self.mass_degeneration)?;
        writeln!(f, "  mass_extinction: {:?}", self.mass_extinction)?;

        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
