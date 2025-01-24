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
    pub cache_hit_miss_pointer: Arc<RwLock<(usize, usize)>>,
    pub track_hit_miss: bool,
}

impl CachePointer {
    pub fn new(cache_size: usize, track_hit_miss: bool) -> Self {
        let non_zero_cache_size = NonZeroUsize::new(cache_size).unwrap();
        let cache = LruCache::with_hasher(non_zero_cache_size, LruCacheBuildHasher::default());
        let cache_pointer = Arc::new(RwLock::new(cache));
        let cache_hit_miss_pointer = Arc::new(RwLock::new((0, 0)));
        Self {
            cache_size,
            cache_pointer,
            cache_hit_miss_pointer,
            track_hit_miss,
        }
    }

    pub fn read(&self, genes_hash: GenesHash) -> Option<FitnessValue> {
        let value = self
            .cache_pointer
            .read()
            .map(|c| c.peek(&genes_hash).cloned())
            .unwrap();

        if self.track_hit_miss {
            if value.is_some() {
                self.cache_hit_miss_pointer.write().unwrap().0 += 1
            } else {
                self.cache_hit_miss_pointer.write().unwrap().1 += 1
            }
        }

        value
    }
    pub fn write(&self, genes_hash: GenesHash, value: FitnessValue) {
        self.cache_pointer.write().unwrap().put(genes_hash, value);
    }
    pub fn number_of_hits_and_misses(&self) -> (usize, usize) {
        *self.cache_hit_miss_pointer.read().unwrap()
    }
}
