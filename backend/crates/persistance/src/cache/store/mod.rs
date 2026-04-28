mod moka;

use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::Arc;

use crate::cache::settings::CacheSettings;

pub use moka::MokaCache;

/// Object-safe async cache trait used internally.
pub trait CacheStore<K, V>: Send + Sync
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn get<'a>(&'a self, key: K) -> Pin<Box<dyn Future<Output = Option<V>> + Send + 'a>>;

    fn insert<'a>(&'a self, key: K, value: V) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

/// Public wrapper type consumers will use. Hides Arc<dyn CacheStore<..>> internals.
#[derive(Clone)]
pub struct Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: Arc<dyn CacheStore<K, V>>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create a moka-backed cache from settings.
    pub fn moka(settings: CacheSettings) -> Self {
        let inner = MokaCache::<K, V>::new(settings);
        Self { inner: Arc::new(inner) }
    }

    /// Create a cache from any Arc'd `CacheStore` implementation.
    pub fn from_arc(inner: Arc<dyn CacheStore<K, V>>) -> Self {
        Self { inner }
    }

    pub fn get<'a>(&'a self, key: K) -> Pin<Box<dyn Future<Output = Option<V>> + Send + 'a>> {
        self.inner.get(key)
    }

    pub fn insert<'a>(&'a self, key: K, value: V) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        self.inner.insert(key, value)
    }
}
