pub mod elite_test;
pub mod tournament_test;

mod select_test {
    #[cfg(test)]
    use genetic_algorithm::distributed::select::{Select, SelectElite};

    #[test]
    fn survival_sizes_normal_replacement() {
        let select = SelectElite::new(0.0, 0.0); //placeholder
        assert_eq!(
            select.parent_and_offspring_survival_sizes(100, 30, 100, 0.5),
            (70, 30)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(100, 20, 100, 0.5),
            (80, 20)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(60, 100, 100, 0.5),
            (50, 50)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(60, 30, 100, 0.5),
            (60, 30)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(30, 30, 100, 0.5),
            (30, 30)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(30, 100, 100, 0.5),
            (30, 70)
        );
    }

    #[test]
    fn survival_sizes_high_replacement() {
        let select = SelectElite::new(0.0, 0.0); //placeholder
        assert_eq!(
            select.parent_and_offspring_survival_sizes(100, 30, 100, 1.0),
            (70, 30)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(100, 20, 100, 1.0),
            (80, 20)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(60, 100, 100, 1.0),
            (0, 100)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(60, 30, 100, 1.0),
            (60, 30)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(30, 30, 100, 1.0),
            (30, 30)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(30, 100, 100, 1.0),
            (0, 100)
        );
    }

    #[test]
    fn survival_sizes_low_replacement() {
        let select = SelectElite::new(0.0, 0.0); //placeholder
        assert_eq!(
            select.parent_and_offspring_survival_sizes(100, 30, 100, 0.0),
            (100, 0)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(100, 20, 100, 0.0),
            (100, 0)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(60, 100, 100, 0.0),
            (60, 40)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(60, 30, 100, 0.0),
            (60, 30)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(30, 30, 100, 0.0),
            (30, 30)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(30, 100, 100, 0.0),
            (30, 70)
        );
    }

    #[test]
    fn survival_sizes_overflow() {
        let select = SelectElite::new(0.0, 0.0); //placeholder
        assert_eq!(
            select.parent_and_offspring_survival_sizes(100, 30, 10, 0.5),
            (5, 5)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(5, 5, 10, 0.5),
            (5, 5)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(3, 3, 10, 0.5),
            (3, 3)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(3, 100, 10, 0.5),
            (3, 7)
        );
        assert_eq!(
            select.parent_and_offspring_survival_sizes(100, 3, 10, 0.5),
            (7, 3)
        );
    }
}
