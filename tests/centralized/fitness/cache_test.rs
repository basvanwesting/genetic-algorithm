#[cfg(test)]
// use crate::support::*;
use genetic_algorithm::centralized::fitness::cache::Cache;

#[test]
fn standard() {
    let cache = Cache::try_new(3).unwrap();

    cache.write(1, 10);
    cache.write(2, 20);

    // hits
    assert_eq!(cache.read(1), Some(10));
    assert_eq!(cache.read(1), Some(10));
    assert_eq!(cache.read(2), Some(20));
    assert_eq!(cache.hit_miss_stats(), (3, 0, 0.0));

    // misses
    assert_eq!(cache.read(3), None);
    assert_eq!(cache.read(3), None);
    assert_eq!(cache.hit_miss_stats(), (3, 2, 1.5));
}

#[test]
fn zero_cache_size() {
    let cache = Cache::try_new(0);

    assert!(cache.is_err());
    assert_eq!(cache.err(), Some("cache_size must be greater than 0"));
}
