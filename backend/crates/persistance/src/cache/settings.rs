use std::time::Duration;

#[derive(Debug, Clone)]
pub struct CacheSettings {
    pub capacity: u64,
    pub ttl: Duration,
}

impl Default for CacheSettings {
    fn default() -> Self {
        Self { capacity: 1024, ttl: Duration::from_secs(300) }
    }
}
