//! Distributed genetic algorithms where each chromosome owns its genes
//!
//! Use this module for:
//! - Binary, List, Unique, and Range genotypes
//! - Custom genetic operators with direct gene access
//! - Maximum extensibility

pub mod allele;
pub mod chromosome;
pub mod crossover;
pub mod errors;
pub mod extension;
pub mod fitness;
pub mod genotype;
pub mod mutate;
pub mod population;
pub mod select;
pub mod strategy;
