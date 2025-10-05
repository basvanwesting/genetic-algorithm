pub mod dynamic_build;
pub mod dynamic_inspect;
pub mod static_build;
pub mod static_inspect;

#[allow(unused_imports)]
pub use approx::relative_eq;
pub use approx::RelativeEq;
#[allow(unused_imports)]
pub use genetic_algorithm::chromosome::{Chromosome, ChromosomeManager};
#[allow(unused_imports)]
pub use genetic_algorithm::genotype::{
    DynamicRangeGenotype, Genotype, StaticBinaryGenotype, StaticRangeGenotype,
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
    fn static_binary_chromosome() {
        let mut genotype = StaticBinaryGenotype::<4, 10>::builder()
            .with_genes_size(4)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let chromosome = static_build::chromosome(&mut genotype, vec![true, false, true, false]);
        println!("{:#?}", chromosome);
        assert_eq!(
            static_inspect::chromosome(&genotype, &chromosome),
            vec![true, false, true, false]
        );
    }

    #[test]
    fn dynamic_range_chromosome() {
        let mut genotype = DynamicRangeGenotype::<f32>::builder()
            .with_genes_size(3)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let chromosome = dynamic_build::chromosome(&mut genotype, vec![0.1, 0.2, 0.3]);
        println!("{:#?}", chromosome);
        assert_eq!(
            dynamic_inspect::chromosome(&genotype, &chromosome),
            vec![0.1, 0.2, 0.3]
        );
    }

    #[test]
    fn static_range_chromosome() {
        let mut genotype = StaticRangeGenotype::<f32, 3, 10>::builder()
            .with_genes_size(3)
            .with_allele_range(0.0..=1.0)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let chromosome = static_build::chromosome(&mut genotype, vec![0.1, 0.2, 0.3]);
        println!("{:#?}", chromosome);
        assert_eq!(
            static_inspect::chromosome(&genotype, &chromosome),
            vec![0.1, 0.2, 0.3]
        );
    }

    #[test]
    fn population_static_binary() {
        let mut genotype = StaticBinaryGenotype::<3, 10>::builder()
            .with_genes_size(3)
            .build()
            .unwrap();
        genotype.chromosomes_setup();

        let population: Population = static_build::population(
            &mut genotype,
            vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![true, true, true],
            ],
        );
        println!("{:#?}", population);
        assert_eq!(
            static_inspect::population(&genotype, &population),
            vec![
                vec![true, false, true],
                vec![false, true, false],
                vec![true, true, true],
            ]
        );
    }
}
