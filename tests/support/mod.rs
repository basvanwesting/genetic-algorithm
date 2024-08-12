pub mod build;
pub mod inspect;

#[allow(unused_imports)]
pub use approx::relative_eq;
pub use approx::RelativeEq;
#[allow(unused_imports)]
pub use num::BigUint;
#[allow(unused_imports)]
pub use rand::rngs::SmallRng;
#[allow(unused_imports)]
pub use rand::SeedableRng;

#[allow(dead_code)]
pub fn relative_chromosome_eq<T: RelativeEq<Epsilon = T> + Clone + Copy>(
    a: Vec<T>,
    b: Vec<T>,
    epsilon: T,
) -> bool {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .all(|(a, b)| a.relative_eq(b, epsilon, epsilon))
}

#[allow(dead_code)]
pub fn relative_population_eq<T: RelativeEq<Epsilon = T> + Clone + Copy>(
    a: Vec<Vec<T>>,
    b: Vec<Vec<T>>,
    epsilon: T,
) -> bool {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .all(|(a, b)| relative_chromosome_eq(a.to_vec(), b.to_vec(), epsilon))
}
