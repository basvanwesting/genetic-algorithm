//! Genetic Algorithm Library - v0.20.5-cd.0 - This is a major breaking change, but use pre-release tags for now, to keep track of base version
//!
//! This library provides two paradigms for genetic algorithms:
//!
//! # Distributed
//! Each chromosome owns its genes. Use for maximum extensibility.
//! ```ignore
//! use genetic_algorithm::distributed::strategy::evolve::prelude::*;
//! ```
//!
//! # Centralized  
//! Population-wide gene storage. Use for maximum performance.
//! ```ignore
//! use genetic_algorithm::centralized::strategy::evolve::prelude::*;
//! ```
//!
//! Choose ONE paradigm for your application.

pub mod centralized;
pub mod distributed;

// NO re-exports at crate root - force explicit paradigm choice
// Users MUST choose: distributed::prelude::* OR centralized::prelude::*
