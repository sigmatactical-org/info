//! [`SpecsCache`].

#[allow(unused_imports)]
use super::*;
use std::sync::OnceLock;
use tokio::sync::RwLock;

/// TTL cache of the GitHub-backed spec listing.
pub(crate) struct SpecsCache {
    pub(crate) client: reqwest::Client,
    pub(crate) state: RwLock<CacheState>,
}
impl SpecsCache {
    /// Process-wide cache instance.
    pub(crate) fn global() -> &'static SpecsCache {
        static CACHE: OnceLock<SpecsCache> = OnceLock::new();
        CACHE.get_or_init(|| SpecsCache {
            client: reqwest::Client::new(),
            state: RwLock::new(CacheState::empty()),
        })
    }
}
