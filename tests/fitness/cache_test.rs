#[cfg(test)]
// use crate::support::*;
use genetic_algorithm::fitness::cache::SharedCache;

#[test]
fn standard() {
    let shared_cache = SharedCache::new(3);

    shared_cache.write(1, 10);
    shared_cache.write(2, 20);

    // hits
    assert_eq!(shared_cache.read(1), Some(10));
    assert_eq!(shared_cache.read(1), Some(10));
    assert_eq!(shared_cache.read(2), Some(20));
    assert_eq!(shared_cache.hit_miss_stats(), (3, 0, 0.0));

    // misses
    assert_eq!(shared_cache.read(3), None);
    assert_eq!(shared_cache.read(3), None);
    assert_eq!(shared_cache.hit_miss_stats(), (3, 2, 1.5));
}
