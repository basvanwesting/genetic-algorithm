//! The search goal to optimize towards (maximize or minimize).
//!
//! Each problem will usually have its own specific [Fitness] function, therefore you need to
//! implement it yourself. Because the [Fitness] function is specific, it is also bound to the
//! [genotype](crate::genotype) through a trait attribute (no reason to make it generic).
//!
//! See [Fitness] Trait
pub mod placeholders;
pub mod prelude;

use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;
use std::sync::{Arc, Mutex};

/// Use isize for easy handling of scores (ordering, comparing) as floats are tricky in that regard.
pub type FitnessValue = isize;

#[derive(Copy, Clone, Debug)]
pub enum FitnessOrdering {
    Maximize,
    Minimize,
}

/// The fitness function, is implemented as a fitness method object.
///
/// Normally the fitness returns [`Some(FitnessValue)`](FitnessValue) for the chromosome, which can be minimized or
/// maximized in the search strategy (e.g. [Evolve](crate::strategy::evolve::Evolve) or
/// [Permutate](crate::strategy::permutate::Permutate)) by providing the [FitnessOrdering].
///
/// If the fitness returns `None`, the chromosome is assumed invalid and taken last in the [competition](crate::compete) phase (also when minimizing).
///
/// # Example:
/// ```rust
/// use genetic_algorithm::fitness::prelude::*;
///
/// #[derive(Clone, Debug)]
/// pub struct CountTrue;
/// impl Fitness for CountTrue {
///     type Genotype = BinaryGenotype;
///     fn calculate_for_chromosome(&mut self, chromosome: &Chromosome<Self::Genotype>) -> Option<FitnessValue> {
///         Some(chromosome.genes.iter().filter(|&value| *value).count() as FitnessValue)
///     }
/// }
/// ```
pub trait Fitness: Clone + Send + std::fmt::Debug {
    type Genotype: Genotype;
    fn call_for_population(&mut self, population: &mut Population<Self::Genotype>, threads: usize) {
        if threads > 1 {
            self.call_for_population_multi_thread(population, threads);
        } else {
            self.call_for_population_single_thread(population);
        }
    }
    fn call_for_population_single_thread(&mut self, population: &mut Population<Self::Genotype>) {
        population
            .chromosomes
            .iter_mut()
            .filter(|c| c.fitness_score.is_none())
            .for_each(|c| self.call_for_chromosome(c));
    }
    fn call_for_population_multi_thread(
        &mut self,
        population: &mut Population<Self::Genotype>,
        threads: usize,
    ) {
        crossbeam::scope(|s| {
            let mut handles = vec![];
            let mutex = Arc::new(Mutex::new(
                population
                    .chromosomes
                    .iter_mut()
                    .filter(|c| c.fitness_score.is_none()),
            ));

            for _i in 0..threads {
                let mutex = Arc::clone(&mutex);
                let mut fitness = self.clone();
                //println!("spawn thread {}", i);
                let handle = s.spawn(move |_| {
                    while let Some(chromosome) = mutex.lock().unwrap().next() {
                        //println!("call for chromosome in thread {}", i);
                        fitness.call_for_chromosome(chromosome);
                    }
                });
                handles.push(handle);
            }

            //println!("before joins");
            for handle in handles {
                handle.join().unwrap();
            }
            //println!("after joins");
        })
        .unwrap();
    }
    fn call_for_chromosome(&mut self, chromosome: &mut Chromosome<Self::Genotype>) {
        chromosome.fitness_score = self.calculate_for_chromosome(chromosome);
    }
    fn calculate_for_chromosome(
        &mut self,
        chromosome: &Chromosome<Self::Genotype>,
    ) -> Option<FitnessValue>;
}
