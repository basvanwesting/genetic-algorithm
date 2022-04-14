mod support;

#[cfg(test)]
mod builders_tests {
    use crate::support::*;

    #[test]
    fn test_chromosome_binary() {
        let chromosome = build::chromosome(vec![true, false, true, false]);
        println!("{:#?}", chromosome);
        assert_eq!(
            inspect::chromosome(&chromosome),
            vec![true, false, true, false]
        );
    }

    #[test]
    fn test_chromosome_discrete() {
        let chromosome = build::chromosome(vec![3, 4, 5, 6]);
        println!("{:#?}", chromosome);
        assert_eq!(inspect::chromosome(&chromosome), vec![3, 4, 5, 6]);
    }

    #[test]
    fn test_chromosome_continuous() {
        let chromosome = build::chromosome(vec![0.1, 0.2, 0.3]);
        println!("{:#?}", chromosome);
        assert_eq!(inspect::chromosome(&chromosome), vec![0.1, 0.2, 0.3]);
    }

    #[test]
    fn test_population_binary() {
        let population = build::population(vec![
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
