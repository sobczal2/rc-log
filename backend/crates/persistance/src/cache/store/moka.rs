use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;

use crate::cache::settings::CacheSettings;

use super::CacheStore;

/// Moka-based cache implementation for the internal trait.
pub struct MokaCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: moka::future::Cache<K, V>,
}

impl<K, V> MokaCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(settings: CacheSettings) -> Self {
        let inner = moka::future::Cache::<K, V>::builder()
            .max_capacity(settings.capacity)
            .time_to_live(settings.ttl)
            .build();

        Self { inner }
    }
}

impl<K, V> CacheStore<K, V> for MokaCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn get<'a>(&'a self, key: K) -> Pin<Box<dyn Future<Output = Option<V>> + Send + 'a>> {
        Box::pin(async move { self.inner.get(&key).await })
    }

    fn insert<'a>(&'a self, key: K, value: V) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move { self.inner.insert(key, value).await })
    }
}

impl<K, V> Clone for MokaCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}
