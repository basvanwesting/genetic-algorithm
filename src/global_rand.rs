//use rand::prelude::*;
use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::distributions::{Bernoulli, Distribution, Standard, Uniform};
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use std::cell::RefCell;

thread_local!(static SMALL_RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_entropy()));

pub fn sample_bernoulli(dist: &Bernoulli) -> bool {
    SMALL_RNG.with(|rng| dist.sample(&mut *rng.borrow_mut()))
}

pub fn sample_uniform<T: SampleUniform>(dist: &Uniform<T>) -> T {
    SMALL_RNG.with(|rng| dist.sample(&mut *rng.borrow_mut()))
}

pub fn gen<T>() -> T
where
    Standard: Distribution<T>,
{
    SMALL_RNG.with(|rng| rng.borrow_mut().gen::<T>())
}

pub fn gen_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    SMALL_RNG.with(|rng| rng.borrow_mut().gen_range(range))
}

pub fn choose<T>(slice: &Vec<T>) -> &T {
    SMALL_RNG.with(|rng| slice.choose(&mut *rng.borrow_mut()).unwrap())
}

pub fn set_small_rng(new_rng: SmallRng) {
    SMALL_RNG.with(|rng| *rng.borrow_mut() = new_rng);
}

//pub fn get_small_rng() -> &mut SmallRng {
//SMALL_RNG.with(|rng| &mut *rng.borrow_mut())
//}
