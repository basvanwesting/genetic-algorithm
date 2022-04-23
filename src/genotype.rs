use crate::chromosome::Chromosome;
use crate::gene::{BinaryGene, ContinuousGene, DiscreteGene, Gene};
use itertools::Itertools;
use rand::prelude::*;
use std::fmt;

pub trait Genotype<T: Gene>: fmt::Display {
    fn gene_size(&self) -> usize;
    fn gene_values(&self) -> Vec<T>;
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<T>;
    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<T>, rng: &mut R);
}

pub struct BinaryRandomGenotype {
    pub gene_size: usize,
}

impl BinaryRandomGenotype {
    pub fn new() -> Self {
        Self { gene_size: 0 }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }
}

impl Genotype<BinaryGene> for BinaryRandomGenotype {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn gene_values(&self) -> Vec<BinaryGene> {
        vec![true, false]
    }
    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<BinaryGene> {
        let genes: Vec<BinaryGene> = (0..self.gene_size).map(|_| rng.gen()).collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<BinaryGene>, rng: &mut R) {
        let index = rng.gen_range(0..self.gene_size);
        chromosome.genes[index] = !chromosome.genes[index];
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for BinaryRandomGenotype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)
    }
}

pub struct DiscreteRandomGenotype {
    pub gene_size: usize,
    pub gene_values: Vec<DiscreteGene>,
}

impl DiscreteRandomGenotype {
    pub fn new() -> Self {
        Self {
            gene_size: 0,
            gene_values: vec![],
        }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }

    pub fn with_gene_values(mut self, gene_values: Vec<DiscreteGene>) -> Self {
        self.gene_values = gene_values;
        self
    }
}

impl Genotype<DiscreteGene> for DiscreteRandomGenotype {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn gene_values(&self) -> Vec<DiscreteGene> {
        self.gene_values.clone()
    }

    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<DiscreteGene> {
        let genes: Vec<DiscreteGene> = (0..self.gene_size)
            .map(|_| *self.gene_values.choose(rng).unwrap())
            .collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<DiscreteGene>, rng: &mut R) {
        let index = rng.gen_range(0..self.gene_size);
        chromosome.genes[index] = *self.gene_values.choose(rng).unwrap();
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for DiscreteRandomGenotype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)?;
        write!(f, "  gene_values: {}\n", self.gene_values.iter().join(","))
    }
}

pub struct ContinuousRandomGenotype {
    pub gene_size: usize,
}

impl ContinuousRandomGenotype {
    pub fn new() -> Self {
        Self { gene_size: 0 }
    }

    pub fn with_gene_size(mut self, gene_size: usize) -> Self {
        self.gene_size = gene_size;
        self
    }
}

impl Genotype<ContinuousGene> for ContinuousRandomGenotype {
    fn gene_size(&self) -> usize {
        self.gene_size
    }
    fn gene_values(&self) -> Vec<ContinuousGene> {
        vec![]
    }

    fn chromosome_factory<R: Rng>(&self, rng: &mut R) -> Chromosome<ContinuousGene> {
        let genes: Vec<ContinuousGene> = (0..self.gene_size).map(|_| rng.gen()).collect();
        Chromosome::new(genes)
    }

    fn mutate_chromosome<R: Rng>(&self, chromosome: &mut Chromosome<ContinuousGene>, rng: &mut R) {
        let index = rng.gen_range(0..self.gene_size);
        chromosome.genes[index] = rng.gen();
        chromosome.taint_fitness_score();
    }
}

impl fmt::Display for ContinuousRandomGenotype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "genotype:\n")?;
        write!(f, "  gene_size: {}\n", self.gene_size)
    }
}
