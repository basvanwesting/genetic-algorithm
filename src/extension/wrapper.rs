pub use super::mass_deduplication::MassDeduplication as ExtensionMassDeduplication;
pub use super::mass_degeneration::MassDegeneration as ExtensionMassDegeneration;
pub use super::mass_extinction::MassExtinction as ExtensionMassExtinction;
pub use super::mass_genesis::MassGenesis as ExtensionMassGenesis;
pub use super::noop::Noop as ExtensionNoop;
pub use super::Extension;

use crate::genotype::EvolveGenotype;
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper<G: EvolveGenotype> {
    MassDeduplication(ExtensionMassDeduplication<G>),
    MassDegeneration(ExtensionMassDegeneration<G>),
    MassExtinction(ExtensionMassExtinction<G>),
    MassGenesis(ExtensionMassGenesis<G>),
    Noop(ExtensionNoop<G>),
}

impl<G: EvolveGenotype> Extension for Wrapper<G> {
    type Genotype = G;

    /// Legacy method for backward compatibility. Delegates to `after_selection_complete`
    #[allow(deprecated)]
    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MassDeduplication(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MassDegeneration(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MassExtinction(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::MassGenesis(extension) => {
                extension.call(genotype, state, config, reporter, rng)
            }
            Wrapper::Noop(extension) => extension.call(genotype, state, config, reporter, rng),
        }
    }

    fn after_selection_complete<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MassDeduplication(extension) => {
                extension.after_selection_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassDegeneration(extension) => {
                extension.after_selection_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassExtinction(extension) => {
                extension.after_selection_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassGenesis(extension) => {
                extension.after_selection_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::Noop(extension) => {
                extension.after_selection_complete(genotype, state, config, reporter, rng)
            }
        }
    }

    fn after_crossover_complete<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MassDeduplication(extension) => {
                extension.after_crossover_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassDegeneration(extension) => {
                extension.after_crossover_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassExtinction(extension) => {
                extension.after_crossover_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassGenesis(extension) => {
                extension.after_crossover_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::Noop(extension) => {
                extension.after_crossover_complete(genotype, state, config, reporter, rng)
            }
        }
    }

    fn after_mutation_complete<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MassDeduplication(extension) => {
                extension.after_mutation_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassDegeneration(extension) => {
                extension.after_mutation_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassExtinction(extension) => {
                extension.after_mutation_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassGenesis(extension) => {
                extension.after_mutation_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::Noop(extension) => {
                extension.after_mutation_complete(genotype, state, config, reporter, rng)
            }
        }
    }
    fn after_generation_complete<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &mut G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::MassDeduplication(extension) => {
                extension.after_generation_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassDegeneration(extension) => {
                extension.after_generation_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassExtinction(extension) => {
                extension.after_generation_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::MassGenesis(extension) => {
                extension.after_generation_complete(genotype, state, config, reporter, rng)
            }
            Wrapper::Noop(extension) => {
                extension.after_generation_complete(genotype, state, config, reporter, rng)
            }
        }
    }
}

impl<G: EvolveGenotype> From<ExtensionMassDeduplication<G>> for Wrapper<G> {
    fn from(extension: ExtensionMassDeduplication<G>) -> Self {
        Wrapper::MassDeduplication(extension)
    }
}
impl<G: EvolveGenotype> From<ExtensionMassDegeneration<G>> for Wrapper<G> {
    fn from(extension: ExtensionMassDegeneration<G>) -> Self {
        Wrapper::MassDegeneration(extension)
    }
}
impl<G: EvolveGenotype> From<ExtensionMassExtinction<G>> for Wrapper<G> {
    fn from(extension: ExtensionMassExtinction<G>) -> Self {
        Wrapper::MassExtinction(extension)
    }
}
impl<G: EvolveGenotype> From<ExtensionMassGenesis<G>> for Wrapper<G> {
    fn from(extension: ExtensionMassGenesis<G>) -> Self {
        Wrapper::MassGenesis(extension)
    }
}
impl<G: EvolveGenotype> From<ExtensionNoop<G>> for Wrapper<G> {
    fn from(extension: ExtensionNoop<G>) -> Self {
        Wrapper::Noop(extension)
    }
}
