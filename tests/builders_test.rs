mod support;

#[cfg(test)]
mod builders_tests {
    use crate::support::*;

    #[test]
    fn test_chromosome_binary() {
        let chromosome = builders::chromosome_binary(vec![true, false, true, false]);
        println!("{:#?}", chromosome);
        //assert_eq!(1, 0);
    }

    #[test]
    fn test_population_binary() {
        let population = builders::population_binary(vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, false],
            vec![false, false, false],
        ]);
        println!("{:#?}", population);
        //assert_eq!(1, 0);
    }
}
