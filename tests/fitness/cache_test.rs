#[cfg(test)]
// use crate::support::*;
use genetic_algorithm::fitness::cache::CachePointer;

#[test]
fn standard() {
    let cache_pointer = CachePointer::new(3);

    cache_pointer.write(1, 10);
    cache_pointer.write(2, 20);

    // hits
    assert_eq!(cache_pointer.read(1), Some(10));
    assert_eq!(cache_pointer.read(1), Some(10));
    assert_eq!(cache_pointer.read(2), Some(20));
    assert_eq!(cache_pointer.hit_miss_stats(), (3, 0, 0.0));

    // misses
    assert_eq!(cache_pointer.read(3), None);
    assert_eq!(cache_pointer.read(3), None);
    assert_eq!(cache_pointer.hit_miss_stats(), (3, 2, 1.5));
}
