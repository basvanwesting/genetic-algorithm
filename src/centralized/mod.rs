//! Centralized genetic algorithms with population-wide gene storage
//!
//! Use this module for:
//! - DynamicRange and StaticRange genotypes
//! - GPU/SIMD-ready operations
//! - Maximum performance with large populations

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
