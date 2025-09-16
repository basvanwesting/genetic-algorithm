pub mod build;
pub mod inspect;

#[allow(unused_imports)]
pub use approx::relative_eq;
pub use approx::RelativeEq;
#[allow(unused_imports)]
pub use genetic_algorithm::chromosome::Chromosome;
#[allow(unused_imports)]
pub use genetic_algorithm::genotype::{
    BinaryGenotype, Genotype, ListGenotype, RangeGenotype,
};
#[allow(unused_imports)]
pub use genetic_algorithm::population::Population;
#[allow(unused_imports)]
pub use num::BigUint;
#[allow(unused_imports)]
pub use rand::rngs::SmallRng;
#[allow(unused_imports)]
pub use rand::SeedableRng;

#[allow(dead_code)]
pub fn relative_chromosome_eq<T: RelativeEq<Epsilon = T> + Clone + Copy + std::fmt::Debug>(
    a: Vec<T>,
    b: Vec<T>,
    epsilon: T,
) -> bool {
    let result = if a.len() == b.len() {
        a.iter()
            .zip(b.iter())
            .all(|(a, b)| a.relative_eq(b, epsilon, epsilon))
    } else {
        false
    };
    if result {
        true
    } else {
        println!("{:?} <> {:?}", a, b);
        false
    }
}

#[allow(dead_code)]
pub fn relative_population_eq<T: RelativeEq<Epsilon = T> + Clone + Copy + std::fmt::Debug>(
    a: Vec<Vec<T>>,
    b: Vec<Vec<T>>,
    epsilon: T,
) -> bool {
    let result = if a.len() == b.len() {
        a.iter()
            .zip(b.iter())
            .all(|(a, b)| relative_chromosome_eq(a.to_vec(), b.to_vec(), epsilon))
    } else {
        println!("{:?} <> {:?}", a, b);
        false
    };
    if result {
        true
    } else {
        println!("{:?} <> {:?}", a, b);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chromosome_binary() {
        let chromosome: Chromosome<bool> = build::chromosome(vec![true, false, true, false]);
        println!("{:#?}", chromosome);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![true, false, true, false]
        );
    }

    #[test]
    fn chromosome_list() {
        let chromosome: Chromosome<u8> = build::chromosome(vec![3, 4, 5, 6]);
        println!("{:#?}", chromosome);
        assert_eq!(inspect::chromosome(&chromosome), vec![3, 4, 5, 6]);
    }

    #[test]
    fn chromosome_range() {
        let chromosome: Chromosome<f32> = build::chromosome(vec![0.1, 0.2, 0.3]);
        println!("{:#?}", chromosome);
        assert_eq!(inspect::chromosome(&chromosome), vec![0.1, 0.2, 0.3]);
    }

    #[test]
    fn population_binary() {
        let population: Population<bool> = build::population(vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, false],
            vec![false, false, false],
        ]);
        println!("{:#?}", population);
        assert_eq!(
            inspect::population(&population),
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false, false],
                vec![false, false, false],
            ]
        );
    }
}
