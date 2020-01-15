pub struct CacheManager;

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {}
    }

    /// TODO: Read cache
    pub fn read_bytes(&self) -> Option<u8> {
        None
    }

    pub fn bunch_read_bytes(&self) -> Vec<u8> {
        vec![]
    }

    pub fn save_bytes(&self, bytes: Vec<u8>) -> Vec<u8> {
        bytes
    }
}
