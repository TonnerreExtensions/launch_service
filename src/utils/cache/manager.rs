use crate::utils::serde::deserializer::Deserializable;
use crate::utils::serde::serializer::Serializable;

pub struct CacheManager;

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {}
    }

    /// TODO: Read cache
    pub async fn read<S: Deserializable>(&self) -> Option<S> {
        None
    }

    pub async fn bunch_read<S: Deserializable>(&self) -> Vec<S> {
        vec![]
    }

    pub async fn save<S: Serializable>(&self, data: S) -> S {
        data
    }

    pub async fn bunch_save<S: Serializable>(&self, data: Vec<S>) -> Vec<S> {
        let mut res = vec![];
        for datum in data {
            res.push(self.save(datum).await);
        }
        res
    }
}
