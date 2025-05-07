mod support;
mod select {
    mod elite_test;
    mod tournament_test;
}
mod select_test {
    #[cfg(test)]
    use genetic_algorithm::select::{Select, SelectElite};

    #[test]
    fn survival_sizes_normal_replacement() {
        let select = SelectElite::new(0.0, 0.0, None); //placeholder
        assert_eq!(select.survival_sizes(100, 30, 100, 0.5), (70, 30));
        assert_eq!(select.survival_sizes(100, 20, 100, 0.5), (80, 20));
        assert_eq!(select.survival_sizes(80, 30, 100, 0.5), (70, 30));
        assert_eq!(select.survival_sizes(30, 30, 100, 0.5), (30, 30));
    }

    #[test]
    fn survival_sizes_high_replacement() {
        let select = SelectElite::new(0.0, 0.0, None); //placeholder
        assert_eq!(select.survival_sizes(100, 30, 100, 1.0), (70, 30));
        assert_eq!(select.survival_sizes(100, 20, 100, 1.0), (80, 20));
        assert_eq!(select.survival_sizes(60, 30, 100, 1.0), (60, 30));
        assert_eq!(select.survival_sizes(30, 30, 100, 1.0), (30, 30));
    }

    #[test]
    fn survival_sizes_low_replacement() {
        let select = SelectElite::new(0.0, 0.0, None); //placeholder
        assert_eq!(select.survival_sizes(100, 30, 100, 0.0), (100, 0));
        assert_eq!(select.survival_sizes(90, 30, 100, 0.0), (90, 10));
        assert_eq!(select.survival_sizes(60, 30, 100, 0.0), (60, 30));
        assert_eq!(select.survival_sizes(30, 30, 100, 0.0), (30, 30));
    }

    #[test]
    fn survival_sizes_overflow() {
        let select = SelectElite::new(0.0, 0.0, None); //placeholder
        assert_eq!(select.survival_sizes(100, 30, 10, 0.5), (50, 0));
        assert_eq!(select.survival_sizes(90, 30, 10, 0.5), (45, 0));
        assert_eq!(select.survival_sizes(60, 30, 10, 0.5), (30, 0));
        assert_eq!(select.survival_sizes(30, 30, 10, 0.5), (15, 0));
    }
}
