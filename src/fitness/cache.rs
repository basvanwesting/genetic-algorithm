use super::FitnessValue;
use crate::chromosome::GenesHash;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
// use std::{thread, time};

#[derive(Debug, Clone)]
pub struct CachePointer {
    pub cache_size: usize,
    pub cache_pointer: Arc<RwLock<LruCache<GenesHash, FitnessValue>>>,
    pub cache_hit_miss_pointer: Arc<RwLock<(usize, usize)>>,
}

impl CachePointer {
    pub fn new(cache_size: usize) -> Self {
        let non_zero_cache_size = NonZeroUsize::new(cache_size).unwrap();
        let cache: LruCache<GenesHash, FitnessValue> = LruCache::new(non_zero_cache_size);
        let cache_pointer = Arc::new(RwLock::new(cache));
        let cache_hit_miss_pointer = Arc::new(RwLock::new((0, 0)));
        Self {
            cache_size,
            cache_pointer,
            cache_hit_miss_pointer,
        }
    }

    pub fn read(&self, genes_hash: GenesHash) -> Option<FitnessValue> {
        self.cache_pointer
            .read()
            .map(|c| c.peek(&genes_hash).cloned())
            .unwrap()
    }
    pub fn write(&self, genes_hash: GenesHash, value: FitnessValue) {
        self.cache_pointer.write().unwrap().put(genes_hash, value);
    }
}
