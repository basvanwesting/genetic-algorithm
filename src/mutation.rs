use crate::chromosome::Chromosome;
use crate::context::Context;
use rand::distributions::{Distribution, Uniform};

pub fn single_gene(context: &Context, chromosome: &mut Chromosome) {
    let gene_range = Uniform::from(0..context.gene_size);
    let mut rng = rand::thread_rng();
    let index = gene_range.sample(&mut rng);
    context.mutate_single_gene(&mut chromosome.genes[index])
}
