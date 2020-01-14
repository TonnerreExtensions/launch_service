use crate::utils::serde::{deserializer::Deserializable, serializer::Serializable};

pub struct CacheManager;

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {}
    }

    /// TODO: Read cache
    pub fn read<C: Deserializable>(&self) -> Option<C> {
        None
    }

    pub fn bunch_read<C: Deserializable>(&self) -> Vec<C> {
        vec![]
    }

    /// TODO: write cache
    pub fn save<C: Serializable>(&self, cache: C) -> C {
        cache
    }

    pub fn bunch_save<C: Serializable>(&self, cache: Vec<C>) -> Vec<C> {
        cache.into_iter()
            .map(|cache| self.save(cache))
            .collect()
    }
}
