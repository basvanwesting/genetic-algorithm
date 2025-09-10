pub use super::elite::Elite as SelectElite;
pub use super::tournament::Tournament as SelectTournament;
pub use super::Select;

use crate::centralized::chromosome::Chromosome;
use crate::centralized::genotype::EvolveGenotype;
use crate::centralized::strategy::evolve::{EvolveConfig, EvolveState};
use crate::centralized::strategy::StrategyReporter;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub enum Wrapper {
    Elite(SelectElite),
    Tournament(SelectTournament),
}

impl Select for Wrapper {
    fn call<G: EvolveGenotype, R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Elite(select) => select.call(genotype, state, config, reporter, rng),
            Wrapper::Tournament(select) => select.call(genotype, state, config, reporter, rng),
        }
    }

    fn extract_elite_chromosomes<G: EvolveGenotype>(
        &self,
        state: &mut EvolveState,
        config: &EvolveConfig,
        elitism_rate: f32,
    ) -> Vec<Chromosome> {
        match self {
            Wrapper::Elite(select) => select.extract_elite_chromosomes::<G>(state, config, elitism_rate),
            Wrapper::Tournament(select) => {
                select.extract_elite_chromosomes::<G>(state, config, elitism_rate)
            }
        }
    }

    fn parent_and_offspring_survival_sizes(
        &self,
        parents_size: usize,
        offspring_size: usize,
        target_population_size: usize,
        replacement_rate: f32,
    ) -> (usize, usize) {
        match self {
            Wrapper::Elite(select) => select.parent_and_offspring_survival_sizes(
                parents_size,
                offspring_size,
                target_population_size,
                replacement_rate,
            ),
            Wrapper::Tournament(select) => select.parent_and_offspring_survival_sizes(
                parents_size,
                offspring_size,
                target_population_size,
                replacement_rate,
            ),
        }
    }
}
impl From<SelectElite> for Wrapper {
    fn from(select: SelectElite) -> Self {
        Wrapper::Elite(select)
    }
}
impl From<SelectTournament> for Wrapper {
    fn from(select: SelectTournament) -> Self {
        Wrapper::Tournament(select)
    }
}
