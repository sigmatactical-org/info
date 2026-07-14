//! [`ContentCache`].

#[allow(unused_imports)]
use super::*;
use std::sync::OnceLock;
use tokio::sync::RwLock;

/// TTL cache of the GitHub-backed doc listing.
pub(crate) struct ContentCache {
    pub(crate) client: reqwest::Client,
    pub(crate) state: RwLock<CacheState>,
}
impl ContentCache {
    /// Process-wide cache instance.
    pub(crate) fn global() -> &'static ContentCache {
        static CACHE: OnceLock<ContentCache> = OnceLock::new();
        CACHE.get_or_init(|| ContentCache {
            client: reqwest::Client::new(),
            state: RwLock::new(CacheState::empty()),
        })
    }
}
