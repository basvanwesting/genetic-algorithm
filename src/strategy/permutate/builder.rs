use super::{Permutate, PermutateReporter, PermutateReporterNoop};
use crate::fitness::{Fitness, FitnessOrdering};
use crate::genotype::PermutableGenotype;
use crate::strategy::Strategy;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromBuilderError(pub &'static str);

/// The builder for an Permutate struct.
#[derive(Debug)]
pub struct Builder<
    G: PermutableGenotype,
    F: Fitness<Allele = G::Allele>,
    SR: PermutateReporter<Allele = G::Allele>,
> {
    pub genotype: Option<G>,
    pub fitness: Option<F>,
    pub fitness_ordering: FitnessOrdering,
    pub par_fitness: bool,
    pub replace_on_equal_fitness: bool,
    pub reporter: SR,
    pub rng: SmallRng,
}

impl<G: PermutableGenotype, F: Fitness<Allele = G::Allele>> Default
    for Builder<G, F, PermutateReporterNoop<G::Allele>>
{
    fn default() -> Self {
        Self {
            genotype: None,
            fitness_ordering: FitnessOrdering::Maximize,
            par_fitness: false,
            replace_on_equal_fitness: false,
            fitness: None,
            reporter: PermutateReporterNoop::new(),
            rng: SmallRng::from_entropy(),
        }
    }
}
impl<G: PermutableGenotype, F: Fitness<Allele = G::Allele>>
    Builder<G, F, PermutateReporterNoop<G::Allele>>
{
    pub fn new() -> Self {
        Self::default()
    }
}
impl<
        G: PermutableGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: PermutateReporter<Allele = G::Allele>,
    > Clone for Builder<G, F, SR>
{
    fn clone(&self) -> Self {
        Self {
            genotype: self.genotype.clone(),
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            fitness: self.fitness.clone(),
            reporter: self.reporter.clone(),
            rng: SmallRng::from_entropy(), // don't clone!
        }
    }
}

impl<
        G: PermutableGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: PermutateReporter<Allele = G::Allele>,
    > Builder<G, F, SR>
{
    pub fn build(self) -> Result<Permutate<G, F, SR>, TryFromBuilderError> {
        self.try_into()
    }
    pub fn with_genotype(mut self, genotype: G) -> Self {
        self.genotype = Some(genotype);
        self
    }
    pub fn with_fitness_ordering(mut self, fitness_ordering: FitnessOrdering) -> Self {
        self.fitness_ordering = fitness_ordering;
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
    pub fn with_fitness(mut self, fitness: F) -> Self {
        self.fitness = Some(fitness);
        self
    }
    pub fn with_reporter<SR2: PermutateReporter<Allele = G::Allele>>(
        self,
        reporter: SR2,
    ) -> Builder<G, F, SR2> {
        Builder {
            genotype: self.genotype,
            fitness_ordering: self.fitness_ordering,
            par_fitness: self.par_fitness,
            replace_on_equal_fitness: self.replace_on_equal_fitness,
            fitness: self.fitness,
            reporter,
            rng: self.rng,
        }
    }
}
impl<
        G: PermutableGenotype,
        F: Fitness<Allele = G::Allele>,
        SR: PermutateReporter<Allele = G::Allele>,
    > Builder<G, F, SR>
{
    pub fn call<R: Rng>(self, rng: &mut R) -> Result<Permutate<G, F, SR>, TryFromBuilderError> {
        let mut permutate: Permutate<G, F, SR> = self.try_into()?;
        permutate.call(rng);
        Ok(permutate)
    }
}
