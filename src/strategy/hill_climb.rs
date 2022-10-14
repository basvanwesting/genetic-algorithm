//! A solution strategy for finding the best chromosome, when search space is convex with little local optima or crossover is impossible or inefficient
mod builder;
pub mod prelude;

pub use self::builder::{
    Builder as HillClimbBuilder, TryFromBuilderError as TryFromHillClimbBuilderError,
};

use super::Strategy;
use crate::chromosome::Chromosome;
use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
use crate::genotype::IncrementalGenotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::cell::RefCell;
use std::fmt;
use thread_local::ThreadLocal;

pub type RandomChromosomeProbability = f64;

#[derive(Clone, Debug)]
pub enum HillClimbVariant {
    Stochastic,
    SteepestAscent,
}

/// The HillClimb strategy is an iterative algorithm that starts with an arbitrary solution to a
/// problem, then attempts to find a better solution by making an incremental change to the
/// solution
///
/// There are 2 variants:
/// * [HillClimbVariant::Stochastic]: does not examine all neighbors before deciding how to move.
///   Rather, it selects a neighbor at random, and decides (based on the amount of improvement in
///   that neighbor) whether to move to that neighbor or to examine another
/// * [HillClimbVariant::SteepestAscent]: all neighbours are compared and the closest to the solution is chosen
///
/// The ending conditions are one or more of the following:
/// * target_fitness_score: when the ultimate goal in terms of fitness score is known and reached
/// * max_stale_generations: when the ultimate goal in terms of fitness score is unknown and one depends on some convergion
///   threshold, or one wants a duration limitation next to the target_fitness_score
/// * min_scale: when the scaling drops below the precision and further refining is useless
///
/// The fitness is calculated each round:
/// * If the fitness is worse
///     * the mutation is ignored and the next round is started based on the current best chromosome
///     * if the scaling is set, the scale is reduced to zoom in on the local solution
///     * the stale generation counter is incremented (functionally)
/// * If the fitness is equal
///     * the mutated chromosome is taken for the next round.
///     * if the scaling is set, the scale is reduced to zoom in on the local solution
///     * the stale generation counter is incremented (functionally)
/// * If the fitness is better
///     * the mutated chromosome is taken for the next round.
///     * if the scaling is set, the scale is reset to its base scale
///     * the stale generation counter is reset (functionally)
///
/// To avoid a local optimum, the `random_chromosome_probability` can be provided.
/// It seems much more efficient to insert random chromosomes in a single [HillClimb] run, than to
/// `call_repeatedly` from the [HillClimbBuilder].
///
/// See [HillClimbBuilder] for initialization options.
///
/// Example:
/// ```
/// use genetic_algorithm::strategy::hill_climb::prelude::*;
/// use genetic_algorithm::fitness::placeholders::SumContinuousGenotype;
///
/// // the search space
/// let genotype = ContinuousGenotype::builder() // f32 alleles
///     .with_genes_size(16)                     // 16 genes
///     .with_allele_range(0.0..1.0)             // values betwee 0.0 and 1.0
///     .with_allele_neighbour_range(-0.1..0.1)  // neighbouring step size or 0.1 in both directions
///     .build()
///     .unwrap();
///
/// // the search strategy
/// let mut rng = rand::thread_rng(); // unused randomness provider implementing Trait rand::Rng
/// let hill_climb = HillClimb::builder()
///     .with_genotype(genotype)
///     .with_variant(HillClimbVariant::SteepestAscent) // check all neighbours for each round
///     .with_fitness(SumContinuousGenotype(1e-5))      // sum the gene values of the chromosomes with precision 0.00001, which means multiply fitness score (isize) by 100_000
///     .with_fitness_ordering(FitnessOrdering::Minimize) // aim for the lowest sum
///     .with_multithreading(true)                 // use all cores for calculating the fitness of the neighbouring_population (only used with HillClimbVariant::SteepestAscent)
///     .with_scaling((1.0, 0.8, 1e-5))            // start with neighbouring mutation scale 1.0 and multiply by 0.8 to zoom in on solution when stale, halt at 1e-5 scale
///     .with_target_fitness_score(10)             // ending condition if sum of genes is <= 0.00010 in the best chromosome
///     .with_valid_fitness_score(100)             // block ending conditions until at least the sum of genes <= 0.00100 is reached in the best chromosome
///     .with_max_stale_generations(1000)          // stop searching if there is no improvement in fitness score for 1000 generations
///     .with_random_chromosome_probability(0.1)   // try a random chromosome with probability 0.1 to avoid local optimum
///     .call(&mut rng)
///     .unwrap();
///
/// // it's all about the best chromosome after all
/// let best_chromosome = hill_climb.best_chromosome().unwrap();
/// assert_eq!(best_chromosome.genes.into_iter().map(|v| v <= 1e-3).collect::<Vec<_>>(), vec![true; 16])
/// ```
pub struct HillClimb<G: IncrementalGenotype, F: Fitness<Genotype = G>> {
    genotype: G,
    fitness: F,
    variant: HillClimbVariant,

    fitness_ordering: FitnessOrdering,
    multithreading: bool,
    max_stale_generations: Option<usize>,
    target_fitness_score: Option<FitnessValue>,
    valid_fitness_score: Option<FitnessValue>,
    random_chromosome_probability: RandomChromosomeProbability,
    scaling: Option<(f32, f32, f32)>,

    pub current_iteration: usize,
    pub current_generation: usize,
    pub current_scaling: Option<f32>,
    pub best_generation: usize,
    best_chromosome: Option<Chromosome<G>>,
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> Strategy<G> for HillClimb<G, F> {
    fn call<R: Rng>(&mut self, rng: &mut R) {
        self.current_generation = 0;
        self.reset_scaling();
        self.best_generation = 0;
        self.best_chromosome = Some(self.genotype.chromosome_factory(rng));
        let random_chromosome_sampler = Bernoulli::new(self.random_chromosome_probability).unwrap();

        let mut fitness_thread_local: Option<ThreadLocal<RefCell<F>>> = None;
        if self.multithreading {
            fitness_thread_local = Some(ThreadLocal::new());
        }

        while !self.is_finished() {
            self.current_generation += 1;
            if random_chromosome_sampler.sample(rng) {
                let working_chromosome = &mut self.genotype.chromosome_factory(rng);
                self.fitness.call_for_chromosome(working_chromosome);
                self.update_best_chromosome(working_chromosome);
            } else {
                match self.variant {
                    HillClimbVariant::Stochastic => {
                        let working_chromosome = &mut self.best_chromosome().unwrap();
                        self.genotype.mutate_chromosome_neighbour(
                            working_chromosome,
                            self.current_scaling,
                            rng,
                        );
                        self.fitness.call_for_chromosome(working_chromosome);
                        self.update_best_chromosome(working_chromosome);
                        self.report_working_chromosome(working_chromosome);
                    }
                    HillClimbVariant::SteepestAscent => {
                        let working_chromosome = &mut self.best_chromosome().unwrap();
                        let working_population = &mut self
                            .genotype
                            .neighbouring_population(working_chromosome, self.current_scaling);

                        self.fitness
                            .call_for_population(working_population, fitness_thread_local.as_ref());
                        self.report_neighbouring_population(working_population);

                        // shuffle, so we don't repeatedly take the same best chromosome
                        working_population.chromosomes.shuffle(rng);
                        self.update_best_chromosome(
                            working_population
                                .best_chromosome(self.fitness_ordering)
                                .unwrap_or(working_chromosome),
                        );
                    }
                }
            }
            self.report_round();
        }
    }
    fn best_chromosome(&self) -> Option<Chromosome<G>> {
        self.best_chromosome.clone()
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> HillClimb<G, F> {
    pub fn builder() -> HillClimbBuilder<G, F> {
        HillClimbBuilder::new()
    }

    fn update_best_chromosome(&mut self, contending_best_chromosome: &Chromosome<G>) {
        self.scale_down();
        match self.best_chromosome.as_ref() {
            None => {
                self.best_chromosome = Some(contending_best_chromosome.clone());
            }
            Some(current_best_chromosome) => {
                match (
                    current_best_chromosome.fitness_score,
                    contending_best_chromosome.fitness_score,
                ) {
                    (None, None) => {}
                    (Some(_), None) => {}
                    (None, Some(_)) => {
                        self.best_chromosome = Some(contending_best_chromosome.clone());
                        self.best_generation = self.current_generation;
                        self.reset_scaling();
                    }
                    (Some(current_fitness_score), Some(contending_fitness_score)) => {
                        match self.fitness_ordering {
                            FitnessOrdering::Maximize => {
                                if contending_fitness_score >= current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    if contending_fitness_score > current_fitness_score {
                                        self.best_generation = self.current_generation;
                                        self.reset_scaling();
                                    }
                                }
                            }
                            FitnessOrdering::Minimize => {
                                if contending_fitness_score <= current_fitness_score {
                                    self.best_chromosome = Some(contending_best_chromosome.clone());
                                    if contending_fitness_score < current_fitness_score {
                                        self.best_generation = self.current_generation;
                                        self.reset_scaling();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.allow_finished_by_valid_fitness_score()
            && (self.is_finished_by_max_stale_generations()
                || self.is_finished_by_target_fitness_score()
                || self.is_finished_by_min_scale())
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

    fn is_finished_by_min_scale(&self) -> bool {
        if let Some(current_scaling) = self.current_scaling {
            current_scaling < self.scaling.as_ref().unwrap().2
        } else {
            false
        }
    }

    fn report_round(&self) {
        log::debug!(
            "generation (current/best): {}/{}, fitness score (best): {:?}, current scale: {:?}",
            self.current_generation,
            self.best_generation,
            self.best_fitness_score(),
            self.current_scaling.as_ref(),
        );
        log::trace!(
            "best - fitness score: {:?}, genes: {:?}",
            self.best_fitness_score(),
            self.best_chromosome
                .as_ref()
                .map_or(vec![], |c| c.genes.clone()),
        );
    }

    fn report_working_chromosome(&self, chromosome: &Chromosome<G>) {
        log::trace!(
            "working - fitness score: {:?}, genes: {:?}",
            chromosome.fitness_score,
            chromosome.genes,
        );
    }

    fn report_neighbouring_population(&self, population: &Population<G>) {
        population.chromosomes.iter().for_each(|chromosome| {
            log::trace!(
                "neighbour - fitness score: {:?}, genes: {:?}",
                chromosome.fitness_score,
                chromosome.genes,
            );
        })
    }

    fn best_fitness_score(&self) -> Option<FitnessValue> {
        self.best_chromosome.as_ref().and_then(|c| c.fitness_score)
    }

    fn reset_scaling(&mut self) {
        self.current_scaling = self.scaling.map(|(base, _factor, _min)| base);
    }

    fn scale_down(&mut self) {
        if let Some(current_scaling) = self.current_scaling {
            self.current_scaling = Some(current_scaling * self.scaling.as_ref().unwrap().1);
        }
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> TryFrom<HillClimbBuilder<G, F>>
    for HillClimb<G, F>
{
    type Error = TryFromHillClimbBuilderError;

    fn try_from(builder: HillClimbBuilder<G, F>) -> Result<Self, Self::Error> {
        if builder.genotype.is_none() {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires a Genotype",
            ))
        } else if builder.fitness.is_none() {
            Err(TryFromHillClimbBuilderError("HillClimb requires a Fitness"))
        } else if builder.max_stale_generations.is_none()
            && builder.target_fitness_score.is_none()
            && builder.scaling.is_none()
        {
            Err(TryFromHillClimbBuilderError(
                "HillClimb requires at least a max_stale_generations, target_fitness_score or scaling ending condition",
            ))
        } else {
            let genotype = builder.genotype.unwrap();

            Ok(Self {
                genotype: genotype,
                fitness: builder.fitness.unwrap(),
                variant: builder.variant.unwrap_or(HillClimbVariant::Stochastic),

                fitness_ordering: builder.fitness_ordering,
                multithreading: builder.multithreading,
                max_stale_generations: builder.max_stale_generations,
                target_fitness_score: builder.target_fitness_score,
                valid_fitness_score: builder.valid_fitness_score,
                random_chromosome_probability: builder.random_chromosome_probability.unwrap_or(0.0),
                scaling: builder.scaling,

                current_iteration: 0,
                current_generation: 0,
                current_scaling: None,
                best_generation: 0,
                best_chromosome: None,
            })
        }
    }
}

impl<G: IncrementalGenotype, F: Fitness<Genotype = G>> fmt::Display for HillClimb<G, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "hill_climb:")?;
        writeln!(f, "  genotype: {:?}", self.genotype)?;
        writeln!(f, "  fitness: {:?}", self.fitness)?;

        writeln!(
            f,
            "  max_stale_generations: {:?}",
            self.max_stale_generations
        )?;
        writeln!(f, "  valid_fitness_score: {:?}", self.valid_fitness_score)?;
        writeln!(f, "  target_fitness_score: {:?}", self.target_fitness_score)?;
        writeln!(f, "  fitness_ordering: {:?}", self.fitness_ordering)?;
        writeln!(f, "  multithreading: {:?}", self.multithreading)?;
        writeln!(f, "  scaling: {:?}", self.scaling)?;
        writeln!(f, "  current iteration: {:?}", self.current_iteration)?;
        writeln!(f, "  current generation: {:?}", self.current_generation)?;
        writeln!(f, "  best fitness score: {:?}", self.best_fitness_score())?;
        writeln!(f, "  best_chromosome: {:?}", self.best_chromosome.as_ref())
    }
}
