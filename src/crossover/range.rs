use super::{Crossover, KeepParent};
use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Range(pub KeepParent);
impl Crossover for Range {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        if genotype.is_unique() {
            panic!("Cannot use Crossover::Range for unique genotype");
        }
        let gene_index_sampler = Uniform::from(0..genotype.gene_size());
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

        if self.0 {
            child_chromosomes.append(&mut population.chromosomes);
        }
        Population::new(child_chromosomes)
    }
}
