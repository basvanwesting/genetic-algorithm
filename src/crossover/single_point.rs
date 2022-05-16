use super::{Crossover, KeepParent};
use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

/// Crossover with a single gene position from which on the rest of the genes are taken from the
/// other parent. The gene position is chosen with uniform probability.
/// Optionally keep parents around to compete with children later on.
///
/// Not allowed for unique genotypes as it would not preserve the gene uniqueness in the children.
#[derive(Clone, Debug)]
pub struct SinglePoint(pub KeepParent);
impl Crossover for SinglePoint {
    fn call<T: Genotype, R: Rng>(&self, genotype: &T, population: &mut Population<T>, rng: &mut R) {
        if population.size() < 2 {
            return;
        }
        let gene_index_sampler = Uniform::from(0..genotype.genes_size());
        if self.0 {
            let mut child_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(population.size());

            for chunk in population.chromosomes.chunks(2) {
                if let [father, mother] = chunk {
                    let index = gene_index_sampler.sample(rng);
                    let mut child_father_genes = father.genes.clone();
                    let mut child_mother_genes = mother.genes.clone();

                    let mut child_father_genes_split = child_father_genes.split_off(index);
                    let mut child_mother_genes_split = child_mother_genes.split_off(index);
                    child_father_genes.append(&mut child_mother_genes_split);
                    child_mother_genes.append(&mut child_father_genes_split);

                    // no need to taint_fitness_score as it is initialized with None
                    child_chromosomes.push(Chromosome::new(child_father_genes));
                    child_chromosomes.push(Chromosome::new(child_mother_genes));
                }
            }
            population.chromosomes.append(&mut child_chromosomes);
        } else {
            for chunk in population.chromosomes.chunks_mut(2) {
                if let [father, mother] = chunk {
                    let index = gene_index_sampler.sample(rng);

                    let mut father_genes_split = father.genes.split_off(index);
                    let mut mother_genes_split = mother.genes.split_off(index);
                    father.genes.append(&mut mother_genes_split);
                    mother.genes.append(&mut father_genes_split);

                    mother.taint_fitness_score();
                    father.taint_fitness_score();
                }
            }
        }
    }
}
