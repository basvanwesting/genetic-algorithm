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
use crate::mutate::Mutate;
use crate::population::Population;
use rand::Rng;
use std::fmt;
use std::ops::Range;

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
/// Optionally a degeneration_range can be set. When approacking a (local) optimum in the fitness
/// score, the variation in the population goes down dramatically. This reduces the efficiency, but
/// also has the risk of local optimum lock-in. Set this parameter to simulate a cambrian
/// explosion, where there is only mutation until the population diversity is large enough again.
/// The controlling metric is fitness score standard deviation in the population. The degeneration
/// has a hysteresis switch, where the degeneration is activated at the start bound of the range,
/// and deactivated at the end bound of the range. The lower bound should be around zero or slightly
/// above (meaning no variation left in population). The higher bound is more difficult to configure,
/// as it depends on the fitness function behaviour (expected spread per mutation). So the higher
/// bound is a case by case analysis.
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
///     .with_target_fitness_score(0)           // goal is 0 times true in the best chromosome
///     .with_max_stale_generations(1000)       // stop searching if there is no improvement in fitness score for 1000 generations
///     .with_degeneration_range(0.005..0.995)  // simulate cambrian explosion when reaching a local optimum
///     .with_fitness(CountTrue)                // count the number of true values in the chromosomes
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the least true values
///     .with_crossover(CrossoverUniform(true)) // crossover all individual genes between 2 chromosomes for offspring
///     .with_mutate(MutateRandom(0.2))         // mutate a single gene with a 20% probability per chromosome
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
    fitness_ordering: FitnessOrdering,
    degeneration_range: Option<Range<f32>>,

    pub current_iteration: usize,
    current_generation: usize,
    best_chromosome: Option<Chromosome<G>>,
    degenerate: bool,
    pub best_generation: usize,
}

impl<G: Genotype, M: Mutate, F: Fitness<Genotype = G>, S: Crossover, C: Compete> Strategy<G>
    for Evolve<G, M, F, S, C>
{
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.degenerate = false;
        self.current_generation = 0;
        self.best_generation = 0;
        let population = &mut self.population_factory(rng);

        while !self.is_finished() {
            if self.toggle_degenerate(population) {
                self.mutate.call(&self.genotype, population, rng);
                self.fitness.call_for_population(population);
            } else {
                self.crossover.call(&self.genotype, population, rng);
                self.mutate.call(&self.genotype, population, rng);
                self.fitness.call_for_population(population);
                self.compete
                    .call(population, self.fitness_ordering, self.population_size, rng);
            }

            self.update_best_chromosome(population);
            //self.report_round(population);
            self.current_generation += 1;
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

    fn toggle_degenerate(&mut self, population: &Population<G>) -> bool {
        if let Some(degeneration_range) = self.degeneration_range.as_ref() {
            let fitness_score_stddev = population.fitness_score_stddev();
            if self.degenerate && fitness_score_stddev > degeneration_range.end {
                //println!("### turn degeneration off");
                self.degenerate = false;
            } else if !self.degenerate && fitness_score_stddev < degeneration_range.start {
                //println!("### turn degeneration on");
                self.degenerate = true;
            }
        }
        self.degenerate
    }

    fn is_finished(&self) -> bool {
        self.is_finished_by_max_stale_generations() || self.is_finished_by_target_fitness_score()
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

    #[allow(dead_code)]
    fn report_round(&self, population: &Population<G>) {
        println!(
            "current generation: {}, best fitness score: {:?}, fitness score count: {}, fitness score stddev: {}, degenerate: {}",
            self.current_generation,
            self.best_fitness_score(),
            population.fitness_score_count(),
            population.fitness_score_stddev(),
            self.degenerate,
        );
    }

    pub fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }

    pub fn population_factory<R: Rng>(&mut self, rng: &mut R) -> Population<G> {
        let chromosomes = (0..self.population_size)
            .map(|_| self.genotype.chromosome_factory(rng))
            .collect();
        Population::new(chromosomes)
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
                fitness_ordering: builder.fitness_ordering,
                degeneration_range: builder.degeneration_range,

                current_iteration: 0,
                current_generation: 0,
                best_generation: 0,
                best_chromosome: None,
                degenerate: false,
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
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  degeneration_range: {:?}", self.degeneration_range)?;

        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
