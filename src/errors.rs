/// Error returned when a strategy builder has invalid or missing configuration.
/// Contains a descriptive message about what went wrong (e.g. missing genotype, missing ending condition).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromStrategyBuilderError(pub &'static str);

/// Error returned when a genotype builder has invalid or missing configuration.
/// Contains a descriptive message about what went wrong (e.g. missing genes_size, missing allele_range).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromGenotypeBuilderError(pub &'static str);
