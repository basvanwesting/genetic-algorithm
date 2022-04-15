mod support;

#[cfg(test)]
mod competition_tests {
    use crate::support::*;
    use genetic_algorithm::competition;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::fitness::{Fitness, FitnessSimpleSum};

    #[test]
    fn test_tournament() {
        let rng = SmallRng::seed_from_u64(0);
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_tournament_size(4)
            .with_rng(rng);

        let mut population = build::population(vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]);

        FitnessSimpleSum.call_for_population(&mut population);
        let new_population = competition::tournament(&mut context, population);

        assert_eq!(
            inspect::population(&new_population),
            vec![
                vec![false, true, true],
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false, true],
            ]
        );
    }

    #[test]
    fn test_tournament_shortage() {
        let mut context = Context::new()
            .with_gene_size(3)
            .with_gene_values(vec![true, false])
            .with_population_size(4)
            .with_tournament_size(4);

        let mut population =
            build::population(vec![vec![false, false, false], vec![false, false, true]]);

        FitnessSimpleSum.call_for_population(&mut population);
        let new_population = competition::tournament(&mut context, population);

        assert_eq!(
            inspect::population(&new_population),
            vec![vec![false, false, true], vec![false, false, false],]
        );
    }
}
