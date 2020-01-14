use crate::utils::cache::cacheable::Cacheable;

pub struct CacheManager;

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {}
    }

    /// TODO: Read cache
    pub fn read<C: Cacheable>(&self) -> Option<C> {
        None
    }

    pub fn bunch_read<C: Cacheable>(&self) -> Vec<C> {
        vec![]
    }

    /// TODO: write cache
    pub fn save<C: Cacheable>(&self, cache: C) -> C {
        cache
    }

    pub fn bunch_save<C: Cacheable>(&self, cache: Vec<C>) -> Vec<C> {
        cache.into_iter()
            .map(|cache| self.save(cache))
            .collect()
    }
}
