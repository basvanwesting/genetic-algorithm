use crate::context::Context;
use crate::population::Population;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;

pub fn mutate_single_gene(context: &Context, population: &mut Population) {
    let gene_range = Uniform::from(0..context.gene_size);
    let mut rng = SmallRng::from_entropy();

    for chromosome in &mut population.chromosomes {
        let index = gene_range.sample(&mut rng);
        context.mutate_single_gene(&mut chromosome.genes[index])
    }
}
