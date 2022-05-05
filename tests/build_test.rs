mod support;

#[cfg(test)]
mod build_tests {
    use crate::support::*;
    use genetic_algorithm::genotype::{BinaryGenotype, ContinuousGenotype, IndexGenotype};

    #[test]
    fn test_chromosome_binary() {
        let chromosome = build::chromosome::<BinaryGenotype>(vec![true, false, true, false]);
        println!("{:#?}", chromosome);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![true, false, true, false]
        );
    }

    #[test]
    fn test_chromosome_index() {
        let chromosome = build::chromosome::<IndexGenotype>(vec![3, 4, 5, 6]);
        println!("{:#?}", chromosome);
        assert_eq!(inspect::chromosome(&chromosome), vec![3, 4, 5, 6]);
    }

    #[test]
    fn test_chromosome_continuous() {
        let chromosome = build::chromosome::<ContinuousGenotype>(vec![0.1, 0.2, 0.3]);
        println!("{:#?}", chromosome);
        assert_eq!(inspect::chromosome(&chromosome), vec![0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_population_binary() {
        let population = build::population::<BinaryGenotype>(vec![
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
