use super::{Crossover, KeepParent};
use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

/// Crossover starting with clones of the parents, with a single gene taken from the other parent.
/// The single gene is chosen with uniform probability.
/// Optionally keep parents around to compete with children later on.
///
/// Not allowed for unique genotypes as it would not preserve the gene uniqueness in the children.
#[derive(Clone, Debug)]
pub struct Single(pub KeepParent);
impl Crossover for Single {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        if population.size() < 2 {
            return population;
        }
        let gene_index_sampler = Uniform::from(0..genotype.gene_size());
        let mut child_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(population.size());

        for chunk in population.chromosomes.chunks(2) {
            if let [father, mother] = chunk {
                let index = gene_index_sampler.sample(rng);
                let mut child_father_genes = father.genes.clone();
                let mut child_mother_genes = mother.genes.clone();

                child_father_genes[index] = mother.genes[index].clone();
                child_mother_genes[index] = father.genes[index].clone();

                // no need to taint_fitness_score as it is initialized with None
                child_chromosomes.push(Chromosome::new(child_father_genes));
                child_chromosomes.push(Chromosome::new(child_mother_genes));
            }
        }

        if self.0 {
            child_chromosomes.append(&mut population.chromosomes);
        }
        Population::new(child_chromosomes)
    }
}
