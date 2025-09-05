#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromStrategyBuilderError(pub &'static str);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TryFromGenotypeBuilderError(pub &'static str);
