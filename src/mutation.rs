use crate::context::Context;
use crate::gene::Gene;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub fn single_gene<T: Gene>(context: &mut Context<T>, population: &mut Population<T>) {
    let gene_range = Uniform::from(0..context.gene_size);

    for chromosome in &mut population.chromosomes {
        let mutation_value: f32 = context.rng.gen();

        if mutation_value <= context.mutation_probability {
            let index = gene_range.sample(&mut context.rng);
            chromosome.genes[index].mutate(context);
        }
    }
}
