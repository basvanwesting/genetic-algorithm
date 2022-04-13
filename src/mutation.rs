use crate::context::Context;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub fn single_gene(context: &Context, population: &mut Population) {
    let gene_range = Uniform::from(0..context.gene_size);
    let mut rng = SmallRng::from_entropy();

    for chromosome in &mut population.chromosomes {
        let mutation_value: f32 = rng.gen();

        if mutation_value <= context.mutation_probability {
            let index = gene_range.sample(&mut rng);
            chromosome.genes[index].mutate();
        }
    }
}
