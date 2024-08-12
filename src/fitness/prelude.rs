#[doc(no_inline)]
pub use crate::chromosome::{Chromosome, GenesKey};
#[doc(no_inline)]
pub use crate::fitness::{Fitness, FitnessOrdering, FitnessValue};
#[doc(no_inline)]
pub use crate::genotype::{
    Allele, BinaryAllele, BinaryGenotype, ContinuousGenotype, DiscreteGenotype, Genotype,
    GenotypeBuilder, MultiContinuousGenotype, MultiDiscreteGenotype, MultiUniqueGenotype,
    TryFromGenotypeBuilderError, UniqueGenotype,
};
