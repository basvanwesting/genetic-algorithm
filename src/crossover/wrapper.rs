pub use super::clone::Clone as CrossoverClone;
pub use super::multi_gene::MultiGene as CrossoverMultiGene;
pub use super::multi_point::MultiPoint as CrossoverMultiPoint;
pub use super::rejuvenate::Rejuvenate as CrossoverRejuvenate;
pub use super::single_gene::SingleGene as CrossoverSingleGene;
pub use super::single_point::SinglePoint as CrossoverSinglePoint;
pub use super::uniform::Uniform as CrossoverUniform;
pub use super::Crossover;

use crate::genotype::{EvolveGenotype, SupportsGeneCrossover, SupportsPointCrossover};
use crate::strategy::evolve::{EvolveConfig, EvolveState};
use crate::strategy::StrategyReporter;
use rand::Rng;

#[derive(Clone, Debug)]
pub enum Wrapper<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover> {
    Clone(CrossoverClone<G>),
    MultiGene(CrossoverMultiGene<G>),
    MultiPoint(CrossoverMultiPoint<G>),
    Rejuvenate(CrossoverRejuvenate<G>),
    SingleGene(CrossoverSingleGene<G>),
    SinglePoint(CrossoverSinglePoint<G>),
    Uniform(CrossoverUniform<G>),
}

impl<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover> Crossover for Wrapper<G> {
    type Genotype = G;

    fn before(&mut self, genotype: &G, state: &mut EvolveState<G>, config: &EvolveConfig) {
        match self {
            Wrapper::Clone(crossover) => crossover.before(genotype, state, config),
            Wrapper::MultiGene(crossover) => crossover.before(genotype, state, config),
            Wrapper::MultiPoint(crossover) => crossover.before(genotype, state, config),
            Wrapper::Rejuvenate(crossover) => crossover.before(genotype, state, config),
            Wrapper::SingleGene(crossover) => crossover.before(genotype, state, config),
            Wrapper::SinglePoint(crossover) => crossover.before(genotype, state, config),
            Wrapper::Uniform(crossover) => crossover.before(genotype, state, config),
        }
    }

    fn call<R: Rng, SR: StrategyReporter<Genotype = G>>(
        &mut self,
        genotype: &G,
        state: &mut EvolveState<G>,
        config: &EvolveConfig,
        reporter: &mut SR,
        rng: &mut R,
    ) {
        match self {
            Wrapper::Clone(crossover) => crossover.call(genotype, state, config, reporter, rng),
            Wrapper::MultiGene(crossover) => crossover.call(genotype, state, config, reporter, rng),
            Wrapper::MultiPoint(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::Rejuvenate(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::SingleGene(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::SinglePoint(crossover) => {
                crossover.call(genotype, state, config, reporter, rng)
            }
            Wrapper::Uniform(crossover) => crossover.call(genotype, state, config, reporter, rng),
        }
    }

    fn after(&mut self, genotype: &G, state: &mut EvolveState<G>, config: &EvolveConfig) {
        match self {
            Wrapper::Clone(crossover) => crossover.after(genotype, state, config),
            Wrapper::MultiGene(crossover) => crossover.after(genotype, state, config),
            Wrapper::MultiPoint(crossover) => crossover.after(genotype, state, config),
            Wrapper::Rejuvenate(crossover) => crossover.after(genotype, state, config),
            Wrapper::SingleGene(crossover) => crossover.after(genotype, state, config),
            Wrapper::SinglePoint(crossover) => crossover.after(genotype, state, config),
            Wrapper::Uniform(crossover) => crossover.after(genotype, state, config),
        }
    }
}

impl<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover> From<CrossoverClone<G>>
    for Wrapper<G>
{
    fn from(crossover: CrossoverClone<G>) -> Self {
        Wrapper::Clone(crossover)
    }
}
impl<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover>
    From<CrossoverMultiGene<G>> for Wrapper<G>
{
    fn from(crossover: CrossoverMultiGene<G>) -> Self {
        Wrapper::MultiGene(crossover)
    }
}
impl<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover>
    From<CrossoverMultiPoint<G>> for Wrapper<G>
{
    fn from(crossover: CrossoverMultiPoint<G>) -> Self {
        Wrapper::MultiPoint(crossover)
    }
}
impl<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover>
    From<CrossoverRejuvenate<G>> for Wrapper<G>
{
    fn from(crossover: CrossoverRejuvenate<G>) -> Self {
        Wrapper::Rejuvenate(crossover)
    }
}
impl<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover>
    From<CrossoverSingleGene<G>> for Wrapper<G>
{
    fn from(crossover: CrossoverSingleGene<G>) -> Self {
        Wrapper::SingleGene(crossover)
    }
}
impl<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover>
    From<CrossoverSinglePoint<G>> for Wrapper<G>
{
    fn from(crossover: CrossoverSinglePoint<G>) -> Self {
        Wrapper::SinglePoint(crossover)
    }
}
impl<G: EvolveGenotype + SupportsGeneCrossover + SupportsPointCrossover>
    From<CrossoverUniform<G>> for Wrapper<G>
{
    fn from(crossover: CrossoverUniform<G>) -> Self {
        Wrapper::Uniform(crossover)
    }
}
