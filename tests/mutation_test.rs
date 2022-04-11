#[cfg(test)]
mod mutation_tests {
    use genetic_algorithm::chromosome::Chromosome;
    use genetic_algorithm::context::Context;
    use genetic_algorithm::mutation::single_gene;

    #[test]
    fn mutate_single_gene() {
        let context = Context::new(3, 10);
        let mut chromosome = Chromosome::new(vec![true, true, true]);

        assert_eq!(chromosome.genes.iter().filter(|&gene| *gene).count(), 3);
        assert_eq!(chromosome.genes.iter().filter(|&gene| !*gene).count(), 0);

        single_gene(&context, &mut chromosome);

        assert_eq!(chromosome.genes.iter().filter(|&gene| *gene).count(), 2);
        assert_eq!(chromosome.genes.iter().filter(|&gene| !*gene).count(), 1);
    }
}
