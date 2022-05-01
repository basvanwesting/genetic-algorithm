use crate::gene::DiscreteGene;
use crate::genotype::{BinaryGenotype, ContinuousGenotype, DiscreteGenotype, Genotype};
use std::any::Any;

pub struct CompositeGenotypeAny {
    pub genotypes: Vec<Box<dyn Any>>,
}

impl CompositeGenotypeAny {
    pub fn new() -> Self {
        Self {
            genotypes: Vec::new(),
        }
    }

    pub fn gene_size(&self) -> usize {
        let mut gene_size = 0;
        for genotype_any in &self.genotypes {
            if let Some(genotype) = genotype_any.downcast_ref::<BinaryGenotype>() {
                gene_size += genotype.gene_size();
                println!("It's a BinaryGenotype: {}", genotype);
            } else if let Some(genotype) =
                genotype_any.downcast_ref::<DiscreteGenotype<DiscreteGene>>()
            {
                gene_size += genotype.gene_size();
                println!("It's a DiscreteGenotypeDiscreteGene: {}", genotype);
            } else if let Some(genotype) = genotype_any.downcast_ref::<ContinuousGenotype>() {
                gene_size += genotype.gene_size();
                println!("It's a ContinuousGenotype: {}", genotype);
            } else {
                println!("Unknown Genotype...");
            }
        }
        gene_size
    }
}

pub enum GenotypeEnum {
    BinaryGenotypeWrapper(BinaryGenotype),
    ContinuousGenotypeWrapper(ContinuousGenotype),
    DiscreteGenotypeDiscreteGeneWrapper(DiscreteGenotype<DiscreteGene>),
}

pub struct CompositeGenotypeEnum {
    pub genotypes: Vec<GenotypeEnum>,
}

impl CompositeGenotypeEnum {
    pub fn new() -> Self {
        Self {
            genotypes: Vec::new(),
        }
    }
    pub fn gene_size(&self) -> usize {
        let mut gene_size = 0;
        for genotype in &self.genotypes {
            match genotype {
                GenotypeEnum::BinaryGenotypeWrapper(genotype) => {
                    gene_size += genotype.gene_size();
                    println!("It's a BinaryGenotype: {}", genotype)
                }
                GenotypeEnum::DiscreteGenotypeDiscreteGeneWrapper(genotype) => {
                    gene_size += genotype.gene_size();
                    println!("It's a DiscreteGenotypeDiscreteGene: {}", genotype)
                }
                GenotypeEnum::ContinuousGenotypeWrapper(genotype) => {
                    gene_size += genotype.gene_size();
                    println!("It's a ContinuousGenotype: {}", genotype)
                } //_ => println!("Unknown Genotype..."),
            }
        }
        gene_size
    }
}
