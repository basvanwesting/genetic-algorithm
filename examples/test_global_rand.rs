use genetic_algorithm::global_rand;
use rand::distributions::{Bernoulli, Distribution, Uniform};
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::Rng;

fn main() {
    let bool_sampler = Bernoulli::new(0.5).unwrap();
    println!(
        "{:?}",
        [
            global_rand::sample_bernoulli(&bool_sampler),
            global_rand::sample_bernoulli(&bool_sampler),
            global_rand::sample_bernoulli(&bool_sampler),
            global_rand::sample_bernoulli(&bool_sampler),
        ]
    );

    let rng = SmallRng::seed_from_u64(0);
    global_rand::set_small_rng(rng);

    println!(
        "{:?}",
        [
            global_rand::sample_bernoulli(&bool_sampler),
            global_rand::sample_bernoulli(&bool_sampler),
            global_rand::sample_bernoulli(&bool_sampler),
            global_rand::sample_bernoulli(&bool_sampler),
        ]
    );

    let int_sampler = Uniform::from(0..10);
    println!(
        "{:?}",
        [
            global_rand::sample_uniform(&int_sampler),
            global_rand::sample_uniform(&int_sampler),
            global_rand::sample_uniform(&int_sampler),
            global_rand::sample_uniform(&int_sampler),
        ]
    );

    println!(
        "{:?}",
        [
            global_rand::gen::<f32>(),
            global_rand::gen::<f32>(),
            global_rand::gen::<f32>(),
            global_rand::gen::<f32>(),
        ]
    );

    let slice = vec![1, 2, 3, 4, 5, 6];
    println!(
        "{:?}",
        [
            global_rand::choose(&slice),
            global_rand::choose(&slice),
            global_rand::choose(&slice),
            global_rand::choose(&slice),
        ]
    );
    //let mut rng = global_rand::get_small_rng();
    //println!("{}", rng.gen::<f64>());
    //println!("{}", rng.gen::<f64>());
    //println!("{}", rng.gen::<f64>());
}
