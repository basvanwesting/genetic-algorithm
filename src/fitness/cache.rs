use super::FitnessValue;
use crate::chromosome::GenesHash;
use lru::LruCache;
use nohash_hasher::NoHashHasher;
use std::hash::BuildHasherDefault;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};

type LruCacheBuildHasher = BuildHasherDefault<NoHashHasher<u64>>;

#[derive(Debug, Clone)]
pub struct CachePointer {
    pub cache_size: usize,
    pub cache_pointer: Arc<RwLock<LruCache<GenesHash, FitnessValue, LruCacheBuildHasher>>>,
    pub cache_hit_pointer: Arc<RwLock<usize>>,
    pub cache_miss_pointer: Arc<RwLock<usize>>,
}

impl CachePointer {
    pub fn new(cache_size: usize) -> Self {
        let non_zero_cache_size = NonZeroUsize::new(cache_size).unwrap();
        let cache = LruCache::with_hasher(non_zero_cache_size, LruCacheBuildHasher::default());
        let cache_pointer = Arc::new(RwLock::new(cache));
        let cache_hit_pointer = Arc::new(RwLock::new(0));
        let cache_miss_pointer = Arc::new(RwLock::new(0));
        Self {
            cache_size,
            cache_pointer,
            cache_hit_pointer,
            cache_miss_pointer,
        }
    }

    pub fn read(&self, genes_hash: GenesHash) -> Option<FitnessValue> {
        let value = self
            .cache_pointer
            .read()
            .map(|c| c.peek(&genes_hash).cloned())
            .unwrap();

        if value.is_some() {
            *self.cache_hit_pointer.write().unwrap() += 1
        } else {
            *self.cache_miss_pointer.write().unwrap() += 1
        }

        value
    }

    pub fn write(&self, genes_hash: GenesHash, value: FitnessValue) {
        self.cache_pointer.write().unwrap().put(genes_hash, value);
    }

    /// hit_miss_stats() -> (hits, misses, ratio)
    pub fn hit_miss_stats(&self) -> (usize, usize, f32) {
        let cache_hits = *self.cache_hit_pointer.read().unwrap();
        let cache_misses = *self.cache_miss_pointer.read().unwrap();

        if cache_misses == 0 {
            (cache_hits, 0, 0.0)
        } else {
            (
                cache_hits,
                cache_misses,
                cache_hits as f32 / cache_misses as f32,
            )
        }
    }
}
