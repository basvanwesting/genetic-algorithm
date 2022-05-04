use super::{Crossover, KeepParent};
use crate::chromosome::Chromosome;
use crate::genotype::Genotype;
use crate::population::Population;
use rand::distributions::{Bernoulli, Distribution};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct All(pub KeepParent);
impl Crossover for All {
    fn call<T: Genotype, R: Rng>(
        &self,
        genotype: &T,
        mut population: Population<T>,
        rng: &mut R,
    ) -> Population<T> {
        if genotype.is_unique() {
            panic!("Cannot use Crossover::All for unique genotype");
        }
        let bool_sampler = Bernoulli::new(0.5).unwrap();
        let mut child_chromosomes: Vec<Chromosome<T>> = Vec::with_capacity(population.size());

        for chunk in population.chromosomes.chunks(2) {
            if let [father, mother] = chunk {
                let mut child_father_genes = father.genes.clone();
                let mut child_mother_genes = mother.genes.clone();

                for index in 0..(genotype.gene_size()) {
                    if bool_sampler.sample(rng) {
                        child_father_genes[index] = mother.genes[index].clone();
                        child_mother_genes[index] = father.genes[index].clone();
                    }
                }

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
