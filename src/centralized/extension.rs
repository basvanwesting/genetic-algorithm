//! When approacking a (local) optimum in the fitness score, the variation in the population goes
//! down dramatically. The offspring will become clones of the parents and the only factor seeding
//! randomness is the mutation of the offspring. But this remaining randomness might not be
//! selected for, killing of the offspring again. This reduces the efficiency, but also has the
//! risk of local optimum lock-in. To increase the variation in the population, an
//! [extension](crate::extension) mechanisms can optionally be used
mod mass_deduplication;
mod mass_degeneration;
mod mass_extinction;
mod mass_genesis;
mod noop;
mod wrapper;

pub use self::mass_deduplication::MassDeduplication as ExtensionMassDeduplication;
pub use self::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use self::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use self::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use self::noop::Noop as ExtensionNoop;
pub use self::wrapper::Wrapper as ExtensionWrapper;

use crate::centralized::chromosome::Chromosome;
use crate::centralized::genotype::EvolveGenotype;
use crate::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use crate::centralized::strategy::StrategyReporter;
use rand::Rng;

pub trait Extension: Clone + Send + Sync + std::fmt::Debug {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    );

    fn extract_elite_chromosomes<G: EvolveGenotype>(
        &self,
        _genotype: &mut G,
        state: &mut EvolveState,
        config: &EvolveConfig,
        elitism_size: usize,
    ) -> Vec<Chromosome> {
        let mut elite_chromosomes: Vec<Chromosome> = Vec::with_capacity(elitism_size);
        for index in state
            .population
            .best_chromosome_indices(elitism_size, config.fitness_ordering)
            .into_iter()
            .rev()
        {
            let chromosome = state.population.chromosomes.swap_remove(index);
            elite_chromosomes.push(chromosome);
        }
        elite_chromosomes
    }

    fn extract_unique_elite_chromosomes<G: EvolveGenotype>(
        &self,
        _genotype: &mut G,
        state: &mut EvolveState,
        config: &EvolveConfig,
        elitism_size: usize,
    ) -> Vec<Chromosome> {
        let mut elite_chromosomes: Vec<Chromosome> = Vec::with_capacity(elitism_size);
        for index in state
            .population
            .best_unique_chromosome_indices(elitism_size, config.fitness_ordering)
            .into_iter()
            .rev()
        {
            let chromosome = state.population.chromosomes.swap_remove(index);
            elite_chromosomes.push(chromosome);
        }
        elite_chromosomes
    }

    fn extract_unique_chromosomes<G: EvolveGenotype>(
        &self,
        _genotype: &mut G,
        state: &mut EvolveState,
        _config: &EvolveConfig,
    ) -> Vec<Chromosome> {
        let mut unique_chromosomes: Vec<Chromosome> = Vec::new();
        for index in state
            .population
            .unique_chromosome_indices()
            .into_iter()
            .rev()
        {
            let chromosome = state.population.chromosomes.swap_remove(index);
            unique_chromosomes.push(chromosome);
        }
        unique_chromosomes
    }
}

#[derive(Clone, Debug)]
pub enum ExtensionEvent {
    MassDeduplication(String),
    MassDegeneration(String),
    MassExtinction(String),
    MassGenesis(String),
}
