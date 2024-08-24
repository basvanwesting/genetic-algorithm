mod support;

#[cfg(test)]
mod playground_tests {
    use crate::support::*;
    use itertools::Itertools;
    use rand::prelude::*;

    #[test]
    fn small_rng_clones_are_not_random() {
        let rng = SmallRng::from_entropy();
        let results: Vec<(f32, f32, f32)> = (0..10)
            .map(|_| rng.clone())
            .map(|mut rng| (rng.gen(), rng.gen(), rng.gen()))
            .dedup()
            .collect();
        println!("{:?}", results);
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn small_rng_from_thread_rng_are_random() {
        let results: Vec<(f32, f32, f32)> = (0..10)
            .map(|_| SmallRng::from_rng(rand::thread_rng()).unwrap())
            .map(|mut rng| (rng.gen(), rng.gen(), rng.gen()))
            .dedup()
            .collect();
        println!("{:?}", results);
        assert_eq!(results.len(), 10);
    }
}
