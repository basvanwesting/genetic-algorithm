mod support;

#[cfg(test)]
mod builders_tests {
    use crate::support::builders::{
        build_chromosome_from_booleans, build_population_from_booleans,
    };

    #[test]
    fn test_chromosome_from_booleans() {
        let chromosome = build_chromosome_from_booleans(vec![true, false, true, false]);
        println!("{:#?}", chromosome);
        //assert_eq!(1, 0);
    }

    #[test]
    fn test_population_from_booleans() {
        let population = build_population_from_booleans(vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, false],
            vec![false, false, false],
        ]);
        println!("{:#?}", population);
        //assert_eq!(1, 0);
    }
}
