use crate::context::Context;
use crate::gene::GeneTrait;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub fn single_gene<T: GeneTrait>(context: &Context<T>, population: &mut Population<T>) {
    let gene_range = Uniform::from(0..context.gene_size);
    let mut rng = SmallRng::from_entropy();

    for _chromosome in &mut population.chromosomes {
        let mutation_value: f32 = rng.gen();

        if mutation_value <= context.mutation_probability {
            let _index = gene_range.sample(&mut rng);
            //chromosome.genes[index].mutate();
        }
    }
}
