use crate::utils::serde::deserializer::Deserializable;
use crate::utils::serde::serializer::Serializable;

pub struct CacheManager;

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {}
    }

    /// TODO: Read cache
    pub fn read<S: Deserializable>(&self) -> Option<S> {
        None
    }

    pub fn bunch_read<S: Deserializable>(&self) -> Vec<S> {
        vec![]
    }

    pub fn save<S: Serializable>(&self, data: S) -> S {
        data
    }

    pub fn bunch_save<S: Serializable>(&self, data: Vec<S>) -> Vec<S> {
        data.into_iter().map(|datum| self.save(datum)).collect()
    }
}
