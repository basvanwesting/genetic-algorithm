use super::FitnessValue;
use crate::distributed::chromosome::GenesHash;
use lru::LruCache;
use nohash_hasher::NoHashHasher;
use std::hash::BuildHasherDefault;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};

type LruCacheBuildHasher = BuildHasherDefault<NoHashHasher<u64>>;

#[derive(Debug, Clone)]
pub struct Cache {
    pub cache_size: usize,
    pub cache_state: Arc<RwLock<LruCache<GenesHash, FitnessValue, LruCacheBuildHasher>>>,
    pub cache_hit_counter: Arc<RwLock<usize>>,
    pub cache_miss_counter: Arc<RwLock<usize>>,
}

impl Cache {
    pub fn try_new(cache_size: usize) -> Result<Self, &'static str> {
        if let Some(non_zero_cache_size) = NonZeroUsize::new(cache_size) {
            let cache = LruCache::with_hasher(non_zero_cache_size, LruCacheBuildHasher::default());
            let cache_state = Arc::new(RwLock::new(cache));
            let cache_hit_counter = Arc::new(RwLock::new(0));
            let cache_miss_counter = Arc::new(RwLock::new(0));
            Ok(Self {
                cache_size,
                cache_state,
                cache_hit_counter,
                cache_miss_counter,
            })
        } else {
            Err("cache_size must be greater than 0")
        }
    }

    pub fn read(&self, genes_hash: GenesHash) -> Option<FitnessValue> {
        let value = self
            .cache_state
            .read()
            .map(|c| c.peek(&genes_hash).cloned())
            .unwrap();

        if value.is_some() {
            *self.cache_hit_counter.write().unwrap() += 1
        } else {
            *self.cache_miss_counter.write().unwrap() += 1
        }

        value
    }

    pub fn write(&self, genes_hash: GenesHash, value: FitnessValue) {
        self.cache_state.write().unwrap().put(genes_hash, value);
    }

    /// hit_miss_stats() -> (hits, misses, ratio)
    pub fn hit_miss_stats(&self) -> (usize, usize, f32) {
        let cache_hits = *self.cache_hit_counter.read().unwrap();
        let cache_misses = *self.cache_miss_counter.read().unwrap();

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
