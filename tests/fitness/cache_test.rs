#[cfg(test)]
// use crate::support::*;
use genetic_algorithm::fitness::cache::CacheReference;

#[test]
fn standard() {
    let cache_reference = CacheReference::new(3);

    cache_reference.write(1, 10);
    cache_reference.write(2, 20);

    // hits
    assert_eq!(cache_reference.read(1), Some(10));
    assert_eq!(cache_reference.read(1), Some(10));
    assert_eq!(cache_reference.read(2), Some(20));
    assert_eq!(cache_reference.hit_miss_stats(), (3, 0, 0.0));

    // misses
    assert_eq!(cache_reference.read(3), None);
    assert_eq!(cache_reference.read(3), None);
    assert_eq!(cache_reference.hit_miss_stats(), (3, 2, 1.5));
}
