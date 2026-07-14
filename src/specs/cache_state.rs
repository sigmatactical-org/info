//! [`CacheState`].

#[allow(unused_imports)]
use super::*;
use std::time::{Duration, Instant};

/// Cached payload plus the instant it was fetched.
pub(crate) struct CacheState {
    pub(crate) documents: Option<Vec<SpecDocumentView>>,
    pub(crate) fetched_at: Option<Instant>,
}
impl CacheState {
    /// Empty, immediately-stale state.
    pub(crate) const fn empty() -> Self {
        Self {
            documents: None,
            fetched_at: None,
        }
    }

    /// Whether the cached payload is still within its TTL.
    pub(crate) fn is_fresh(&self, ttl: Duration) -> bool {
        self.documents
            .as_ref()
            .is_some_and(|_| self.fetched_at.is_some_and(|at| at.elapsed() < ttl))
    }
}
